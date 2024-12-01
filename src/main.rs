use aoc24::{puzzles::day01, Cli, Commands};
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
            Commands::Solve { day } => match day {
                1 => day01::solve(),
                _ => todo!(),
            },
        }
    }
}
