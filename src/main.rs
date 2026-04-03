mod deck;
mod srs;

use clap::{Parser, Subcommand};
use deck::Deck;
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

fn find_repo_root(start: &PathBuf) -> anyhow::Result<PathBuf> {
    let mut dir = std::fs::canonicalize(start)?;
    loop {
        if dir.join("Makefile").exists() && dir.join("problems").exists() {
            return Ok(dir);
        }
        if !dir.pop() {
            anyhow::bail!("couldn't find leetcod repo root");
        }
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let root = find_repo_root(&cli.root)?;
    let mut deck = Deck::load(&root)?;

    match cli.command {
        Cmd::Add { problem } => {
            let problem = match problem {
                Some(p) => p,
                None => todo!("helper function for the most recent problem"),
            };
            if deck.add(problem.clone()) {
                deck.save(&root)?;
                println!("\x1b[32m✓\x1b[0m added '{}' to deck", problem);
            } else {
                println!("{} is already in deck", problem);
            }
        }
        Cmd::Due => todo!(),
        Cmd::Review { problem } => todo!(),
        Cmd::Good => todo!(),
        Cmd::Again => todo!(),
        Cmd::Status => todo!(),
        Cmd::Import => todo!(),
    }

    Ok(())
}
