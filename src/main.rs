use aoc24::{Cli, Commands, DAY_MAP};
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    if let Some(subcommand) = cli.run {
        match subcommand {
            Commands::All => {
                for (idx, solve) in DAY_MAP.iter().enumerate() {
                    println!("Day {:02}", idx + 1);
                    solve();
                    println!();
                }
            }
            Commands::Solve { day } => {
                if (1..=DAY_MAP.len()).contains(&(day.unsigned_abs())) {
                    DAY_MAP[day.unsigned_abs() - 1]();
                } else {
                    println!("Error: Day {day} not implemented");
                }
            }
        }
    }
}
