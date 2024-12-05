pub mod puzzles;
pub mod shared;

use clap::{Parser, Subcommand};

pub const DAY_MAP: [fn(); 5] = [
    puzzles::day01::solve as fn(),
    puzzles::day02::solve as fn(),
    puzzles::day03::solve as fn(),
    puzzles::day04::solve as fn(),
    puzzles::day05::solve as fn(),
];

#[derive(Parser)]
pub struct Cli {
    /// Select which day to do stuff with
    #[command(subcommand)]
    pub run: Option<Commands>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// Run all solutions
    All,
    /// Run the solution for a day
    Solve { day: isize },
}
