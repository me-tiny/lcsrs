mod deck;
mod srs;

use clap::{Parser, Subcommand};
use deck::Deck;
use std::path::PathBuf;
use std::process::Command;
use which::which;

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

fn most_recent_problem(root: &PathBuf) -> anyhow::Result<String> {
    // check if fd exists, otherwise fallback to find
    let mut binding = Command::new("find");
    let cmd = binding.args(["problems", "-name", "sol.cpp", "-printf", "%T@ %p\\n"]);

    if which("fd").is_ok() {
        let cmd = Command::new("fd").args([
            "--glob",
            "sol.cpp",
            "problems",
            "-x",
            "stat",
            "-c",
            "%Y.%09X %n",
        ]);
    }

    let output = cmd.current_dir(root).output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let line = stdout
        .lines()
        .filter(|l| !l.is_empty())
        .max_by(|a, b| {
            let ta: f64 = a
                .split_whitespace()
                .next()
                .unwrap_or("0")
                .parse()
                .unwrap_or(0.0);
            let tb: f64 = b
                .split_whitespace()
                .next()
                .unwrap_or("0")
                .parse()
                .unwrap_or(0.0);
            ta.partial_cmp(&tb).unwrap()
        })
        .ok_or_else(|| anyhow::anyhow!("no solution files found"))?;

    let path = line.split_whitespace().nth(1).unwrap();
    let problem = path
        .strip_prefix("problems/")
        .and_then(|s| s.strip_suffix("/sol.cpp"))
        .ok_or_else(|| anyhow::anyhow!("unexpected path format {}", path))?;

    Ok(problem.to_string())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let root = find_repo_root(&cli.root)?;
    let mut deck = Deck::load(&root)?;

    match cli.command {
        Cmd::Add { problem } => {
            let problem = match problem {
                Some(p) => p,
                None => most_recent_problem(&root)?,
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
