# CLAUDE.md — tarot-cli

This file provides context for AI assistants working in this repository.

## Project Overview

**tarot-cli** is a Rust CLI implementation of the French Tarot card game (78-card deck). The project covers the full game loop: deck creation, dealing, bidding, king calling, kitty composition, trick playing, and score computation.

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
    ├── card_test.rs
    ├── bid_test.rs
    ├── deal_tests.rs
    ├── game_tests.rs
    ├── score_tests.rs
    └── trick_test.rs
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
- `cargo test --verbose` — all tests must pass
- Standard file checks (trailing whitespace, EOF newlines, TOML/YAML/JSON validity, no private keys)

Install hooks: `pre-commit install`

## CI/CD

GitHub Actions (`.github/workflows/rust.yml`) triggers on push/PR to `main` and `develop`:
1. `cargo build --verbose --all-targets`
2. `cargo test --verbose --all-targets`
3. `cargo clippy --all-targets --all-features -- -D warnings`
4. `cargo audit`

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
- **Traits for polymorphism**: Human vs Bot behavior is abstracted via traits (`PlayerActions`, `CardActions`, `DealActions`). Prefer extending traits over adding `if kind == Human` branches.
- **Enums + exhaustive match**: Game states and decisions use enums. Always handle all variants.
- **Recursive iteration**: `collect_bids()`, `play_tricks()`, and `select()` are recursive. Maintain this pattern for consistency.
- **`#[derive(Default)]`**: Used broadly on structs — ensure new fields have sensible defaults.
- **`.clone()` usage**: Noted as a known issue (see `deal.rs:51`, `main.rs:42`). Avoid introducing new `.clone()` calls without justification.

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
| Kitty size | 6 cards (4 players), 3 cards (5+ players) |
| Poignee bonus | Simple(8 trumps)=20, Double(10)=30, Triple(13)=40 |
| Chelem bonus | Announced+succeeded=400, Not announced+succeeded=200, Announced+lost=−200 |
| Petit au bout | Little trump won in last trick = ±10 pts (direction depends on winner side) |
| Default players | 4 (1 human, 3 bots) |

Card scoring: Kings/Oudlers=4.5pts, Queens=3.5pts, Knights=2.5pts, Jacks=1.5pts, all others=0.5pts.

## Known Incomplete Areas (TODOs)

These areas are work-in-progress — be careful not to assume they are fully implemented:

- **Bot AI** (`player.rs:126-127`, `trick.rs:64-65`): Bot king calling and trick card selection are stubs (`unimplemented!()`).
- **Score display** (`main.rs:42`, `main.rs:57-58`): Score output after each deal is incomplete.
- **Chelem state** (`main.rs:35`, `main.rs:54`): Chelem reordering/state tracking after deals needs work.
- **Human poignee/chelem declaration** (`hand.rs:86`, `hand.rs:91-93`): Not yet implemented for human players.
- **Kitty correction** (`kitty.rs:41`): The mechanism for human kitty editing is a stub.
- **Player/game submodules** (`game.rs:92`): Noted split into submodules is pending.
- **`new()` constructors** (`main.rs:12`): Some structs use ad-hoc initialization instead of `new()`.

When working on these areas, check the TODO comments and the official rules PDF for correctness.

## Dependencies

| Crate | Version | Purpose |
|---|---|---|
| `rand` | 0.8.5 | Deck shuffling, bot random decisions |
| `rstest` | 0.22.0 | Parametrized tests (`#[values(...)]`) |

Rust edition: **2021**
