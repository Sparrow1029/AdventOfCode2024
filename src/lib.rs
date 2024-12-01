pub mod puzzles;
pub mod shared;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    /// Run all days
    #[arg(short, long, action)]
    pub all: bool,
    /// Select which day to do stuff with
    #[command(subcommand)]
    pub run: Option<Commands>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// Run the solution for a day
    Solve { day: u32 },
    /// Run the tests for a day
    Test { day: u32 },
}
