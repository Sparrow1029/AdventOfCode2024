use aoc24::{Cli, Commands, DAY_MAP};
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    if cli.all {
        todo!() // Run all solves
    }
    if let Some(subcommand) = cli.run {
        match subcommand {
            Commands::Test { day } => {
                println!("Test day {day:02}")
            }
            Commands::Solve { day } => match DAY_MAP.get(&day) {
                Some(solve) => solve(),
                None => println!("Error: Day {day} not implemented"),
            },
        }
    }
}
