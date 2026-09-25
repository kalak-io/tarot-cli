#[cfg(test)]
mod bench {
    use std::process::Command;

    fn run_bench(args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_bench"))
            .args(args)
            .output()
            .expect("the bench binary runs")
    }

    #[test]
    fn bench_reports_every_player_count_with_zero_sum_scores() {
        let output = run_bench(&["200", "1"]);
        assert!(output.status.success());
        let report = String::from_utf8(output.stderr).unwrap();
        for n_players in 3..=5 {
            assert!(report.contains(&format!(
                "=== {n_players} players: 200 deals played, seed 1"
            )));
        }
        assert_eq!(
            report
                .matches("Deals whose scores do not sum to 0: 0")
                .count(),
            3
        );
    }

    #[test]
    fn bench_with_the_same_seed_gives_the_same_report() {
        // Drop the timing, which changes between runs
        let report = |output: std::process::Output| -> String {
            String::from_utf8(output.stderr)
                .unwrap()
                .lines()
                .filter(|line| !line.starts_with("==="))
                .collect::<Vec<_>>()
                .join("\n")
        };
        assert_eq!(
            report(run_bench(&["100", "7"])),
            report(run_bench(&["100", "7"]))
        );
    }

    #[test]
    fn calibrate_reports_cutoffs_for_every_player_count() {
        let output = run_bench(&["--calibrate", "50", "1"]);
        assert!(output.status.success());
        let report = String::from_utf8(output.stderr).unwrap();
        for n_players in 3..=5 {
            assert!(report.contains(&format!("{n_players} players: Take ")));
        }
    }

    #[test]
    fn bench_rejects_an_invalid_argument() {
        let output = run_bench(&["many"]);
        assert_eq!(output.status.code(), Some(2));
    }
}
