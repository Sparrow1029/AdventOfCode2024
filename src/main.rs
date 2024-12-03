use aoc24::{Cli, Commands, DAY_MAP};
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    if let Some(subcommand) = cli.run {
        match subcommand {
            Commands::All => {
                for k in 1..=DAY_MAP.keys().len() {
                    println!("Day {k:02}");
                    DAY_MAP[&(k as u32)]();
                    println!();
                }
            }
            Commands::Solve { day } => match DAY_MAP.get(&day) {
                Some(solve) => solve(),
                None => println!("Error: Day {day} not implemented"),
            },
        }
    }
}
