# CLAUDE.md — tarot-cli

This file provides context for AI assistants working in this repository.

## Project Overview

**tarot-cli** is a Rust CLI implementation of the French Tarot card game (78-card deck). The project covers the full game loop for 3, 4 or 5 players: deck creation, dealing, bidding, card calling (5 players), kitty composition, trick playing, and score computation.

Official rules reference: `tarot-official-rules.pdf` (authoritative source for game rule questions).

## Repository Structure

```
src/
├── main.rs              # Game loop entry point
├── lib.rs               # Crate root (re-exports common module)
└── common/
    ├── mod.rs           # Module exports
    ├── card.rs          # Card, Suit, CardSuits enum, scoring values
    ├── bid.rs           # Bids enum (Take/Guard/GuardWithout/GuardAgainst/Pass), bot bidding
    ├── deal.rs          # Deal orchestration: distribution, bids, kitty, tricks, score
    ├── game.rs          # Game struct: deck creation, player management, dealer rotation
    ├── hand.rs          # Hand struct: Side (Attack/Defense), Poignee bonus
    ├── player.rs        # Player struct, PlayerKind (Human/Bot), PlayerRole (Dealer/Receiver)
    ├── score.rs         # Score computation, oudler thresholds, bid multipliers
    ├── taker.rs         # Taker struct (Player + Bid)
    ├── kitty.rs         # Kitty (widow) struct, size rules, discard restrictions
    ├── trick.rs         # Trick validation, card play rules, petit au bout detection
    ├── chelem.rs        # Chelem (slam) state and result types
    └── utils.rs         # RNG, circular index, CLI selection, vector helpers
tests/
    ├── bid_test.rs
    ├── card_test.rs
    ├── deal_tests.rs
    ├── game_tests.rs
    ├── hand_tests.rs
    ├── integration_test.rs
    ├── kitty_tests.rs
    ├── player_tests.rs
    ├── score_tests.rs
    ├── trick_test.rs
    └── utils_tests.rs
```

## Development Commands

```bash
# Build
cargo build --all-targets

# Run tests
cargo test --verbose --all-targets

# Format code (required before committing)
cargo fmt

# Lint (warnings are errors in CI)
cargo clippy --all-targets --all-features -- -D warnings

# Security audit
cargo audit

# Run the game
cargo run
```

## Pre-commit Hooks

The repo uses pre-commit hooks (`.pre-commit-config.yaml`). On every commit, these run automatically:
- `cargo fmt` — enforces formatting
- `cargo check --all-targets` — must compile
- `cargo clippy --all-targets --all-features -- -D warnings` — same lint as CI
- `cargo test --verbose` — all tests must pass
- Standard file checks (trailing whitespace, EOF newlines, TOML/YAML/JSON validity, no private keys)

Install hooks: `pre-commit install`

## CI/CD

GitHub Actions (`.github/workflows/rust.yml`) triggers on push to `main`/`develop` and on every PR. Jobs run in parallel:
1. `cargo fmt --check`
2. `cargo build --verbose --all-targets`
3. `cargo test --verbose --all-targets`
4. `cargo clippy --all-targets --all-features -- -D warnings`
5. `cargo audit`

Clippy warnings are treated as errors — fix all lints before pushing.

## Commit Message Convention

The project uses a prefix-based convention (no colon):

```
ADD <description>       # New feature or file
UPDATE <description>    # Modification to existing code
FIX <description>       # Bug fix
REMOVE <description>    # Deletion
RENAME <description>    # Rename only
WIP: <description>      # Work in progress (avoid on main)
```

Examples from history: `ADD official rules`, `UPDATE way to compute score`, `FIX CI`

## Code Conventions

### Naming
- **Structs/Enums/Traits**: `CamelCase` — `Card`, `CardSuits`, `PlayerKind`, `PlayerActions`
- **Functions/Variables**: `snake_case` — `is_superior_than`, `get_available_bids`
- **Constants**: `UPPER_SNAKE_CASE` — `KING_RANK`, `BASE_SCORE`, `TOTAL_CARDS`

### Patterns
- **Human/Bot dispatch**: `impl PlayerActions for Player` is the only place that does `match self.kind`. Each decision is a `human_*`/`bot_*` method pair on the domain type (`Bid`, `Kitty`, `Hand`, `Trick`). A new decision adds a pair there and one match arm in `player.rs`.
- **Enums + exhaustive match**: Game states and decisions use enums. Always handle all variants.
- **Recursive iteration**: `play_tricks()`, `select()` and `select_card()` are recursive. Maintain this pattern for consistency. `collect_bids()` is a single loop, because each player bids once.
- **`#[derive(Default)]`**: Used broadly on structs — ensure new fields have sensible defaults.
- **`.clone()` usage**: A known issue (see `deal.rs:57`). Do not add a new `.clone()` without a reason.
- **Taker copy**: `deal.taker.player` is a copy made at bid time. To change the real player, use `taker_index()` into `deal.players`.

### Testing
- Use `rstest` with `#[rstest]` + `#[values(...)]` for parametrized cases.
- Test files live in `tests/` (integration-style), not inline in `src/`.
- Test function names are descriptive sentences: `deals_right_number_of_cards`, `trick_get_best_played_card_index`.

## Game Domain Reference

Key rules encoded in the codebase:

| Concept | Value |
|---|---|
| Deck size | 78 cards (22 trumps + 14×4 suits) |
| Oudlers | Fool (rank 22), Little (rank 1), Big (rank 21) — all Trumps |
| Score thresholds | 0 oudlers→56pts, 1→51pts, 2→41pts, 3→36pts needed to win |
| Bid multipliers | Take×1, Guard×2, GuardWithout×4, GuardAgainst×6 |
| Base score | 25 points |
| Kitty size | 6 cards (2-4 players), 3 cards (5+ players). Counts for the attack, except on Guard Against |
| Contract won | Points minus threshold >= 0 (exactly met counts as won). With 3 or 5 players, a half point goes to the winning side |
| Poignee bonus | Simple=20, Double=30, Triple=40, paid to the side that wins the deal. Trumps needed: 10/13/15 (4 players), 13/15/18 (3), 8/10/13 (5) |
| Chelem bonus | Announced+succeeded=400, Not announced+succeeded=200, Announced+lost=−200 |
| Petit au bout | Little trump won in last trick = ±10 pts × bid multiplier (sign follows the winning side) |
| Players | 3, 4 or 5, chosen at startup by `ask_player_count()` (Enter picks 4). Player 1 is human, the others are bots |
| Dealing | 24 cards each (3 players, 4 by 4), 18 (4 players, 3 by 3), 15 (5 players, 3 by 3). Kitty cards go one at a time after random packets, never the first or last card of the deck |
| Kitty discard | No oudler and no king |
| Called card (5 players) | The taker calls a king. If they hold all 4 kings, they call a queen, and so on down to a jack (`callable_cards()`). The holder is the partner. A taker who holds the called card, or finds it in the kitty, plays alone |
| First lead (5 players) | Not in the called card's suit, unless it is the called card itself |
| Following | Follow the led suit. Without it, play a trump. When playing a trump, over-trump if possible. With neither, discard any card |
| Excuse (Fool) | Playable any time. It stays with its side, which gives the other side a 0.5-point card in exchange. Played to the last trick, it goes to the winner. It never wins a trick, except when a side that won every trick leads it to the last one (chelem) |
| Team scores | Each defender pays the attack score, a partner gets it once, and the taker gets the rest. The scores sum to 0 |
| Bot bid cutoffs | Hand strength needed for Take/Guard/GuardWithout/GuardAgainst: 50/57/65/72 (3 players), 36/42/50/57 (4), 29/35/43/50 (5). See `bid_cutoffs()` in `bid.rs` |

Card scoring: Kings/Oudlers=4.5pts, Queens=3.5pts, Knights=2.5pts, Jacks=1.5pts, all others=0.5pts.

## Known Incomplete Areas (TODOs)

- **Player/game submodules** (`game.rs:103`): Split into submodules is pending.

## Dependencies

| Crate | Version | Purpose |
|---|---|---|
| `rand` | 0.8.6 | Deck shuffling, cut, kitty positions |
| `rstest` | 0.22.0 | Parametrized tests (`#[values(...)]`), dev-dependency only |

Rust edition: **2021**
