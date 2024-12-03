pub mod puzzles;
pub mod shared;
use std::collections::HashMap;

use clap::{Parser, Subcommand};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DAY_MAP: HashMap<u32, fn()> = HashMap::from([
        (1, puzzles::day01::solve as fn()),
        (2, puzzles::day02::solve as fn()),
        (3, puzzles::day03::solve as fn()),
    ]);
}

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
    Solve { day: u32 },
}
