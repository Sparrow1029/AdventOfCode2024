use aoc24::{Cli, Commands, DAYS};
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    if let Some(subcommand) = cli.run {
        match subcommand {
            Commands::All => {
                for (idx, solve) in DAYS.iter().enumerate() {
                    println!("Day {:02}", idx + 1);
                    solve();
                    println!();
                }
            }
            Commands::Solve { day } => {
                if day >= 1 && (1..DAYS.len() + 1).contains(&day.unsigned_abs()) {
                    DAYS[day.unsigned_abs() - 1]();
                } else {
                    println!("Error: Day {day} not implemented");
                }
            }
        }
    }
}
