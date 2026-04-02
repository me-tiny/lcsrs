mod deck;
mod srs;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "lcsrs", about = "spaced repetition for leetcode")]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
    /// path to leetcode repo root (default: current dir)
    #[arg(long, default_value = ".")]
    root: PathBuf,
}

#[derive(Subcommand)]
enum Cmd {
    /// register a solved problem in to the deck
    Add {
        /// problem directory name, e.g. "0001-two-sum"
        /// defaults to auto detect the most recently modified sol.cpp
        problem: Option<String>,
    },
    /// list problems due for review
    Due,
    /// begin a review session: backs up sol.cpp, clears it, opens editor
    Review {
        /// specific problem to review (default: most overdue)
        problem: Option<String>,
    },
    /// rate current review as Good (interval increases)
    Good,
    /// rate the current review as Again (interval resets to 1 day)
    Again,
    /// show all problem and their SRS state
    Status,
    /// import all existing problems into the deck
    Import,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Cmd::Add { problem } => todo!(),
        Cmd::Due => todo!(),
        Cmd::Review { problem } => todo!(),
        Cmd::Good => todo!(),
        Cmd::Again => todo!(),
        Cmd::Status => todo!(),
        Cmd::Import => todo!(),
    }

    Ok(())
}
