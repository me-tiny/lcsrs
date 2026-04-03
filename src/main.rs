mod deck;
mod srs;

use clap::{Parser, Subcommand};
use deck::Deck;
use srs::Rating;
use std::path::{Path, PathBuf};
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
    let mut cmd = if which("fd").is_ok() {
        let mut c = Command::new("fd");
        c.args([
            "--glob",
            "sol.cpp",
            "problems",
            "-x",
            "stat",
            "-c",
            "%Y.%09X %n",
        ]);
        c
    } else {
        let mut c = Command::new("find");
        c.args(["problems", "-name", "sol.cpp", "-printf", "%T@ %p\\n"]);
        c
    };

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

fn open_editor(root: &Path, problem: &str) {
    let sol_path = root.join("problems").join(problem).join("sol.cpp");
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string());
    let _ = Command::new(editor).arg(&sol_path).status();
}

fn rate_active(root: &Path, deck: &mut Deck, rating: Rating) -> anyhow::Result<()> {
    let problem = deck::active_review(root)?
        .ok_or_else(|| anyhow::anyhow!("no active review, run `lcsrs review first`"))?;

    deck.rate(&problem, rating)?;
    deck.save(root)?;
    deck::finish_review(root)?;

    let card = &deck.cards[deck.find(&problem).unwrap()];
    let label = match rating {
        Rating::Good => "\x1b[32mgood\x1b[0m",
        Rating::Again => "\x1b[31magain\x1b[0m",
    };
    println!(
        "{} '{}' - next review in {} day(s)",
        label,
        problem,
        card.interval.ceil() as i64
    );
    Ok(())
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
        Cmd::Due => {
            let due = deck.due_cards();
            if due.is_empty() {
                println!("nothing due, all caught up");
            } else {
                println!("{} problem(s) due for review:\n", due.len());
                for card in &due {
                    let overdue = card.days_overdue();
                    let overdue_str = if overdue > 0 {
                        format!(" \x1b[31m({}d overdue)\x1b[0m", overdue)
                    } else {
                        " \x1b[33m(due today)\x1b[0m".to_string()
                    };
                    println!(
                        "  {} — streak: {}, reviews: {}{}",
                        card.problem, card.streak, card.reviews, overdue_str
                    );
                }
            }
        }
        Cmd::Review { problem } => {
            // need to check if there's already an active review
            if let Some(active) = deck::active_review(&root)? {
                println!(
                    "review already in progress: {}\n\
                finish it with `lcsrs good` or `lcsrs again`",
                    active
                );
                return Ok(());
            }

            let problem = match problem {
                Some(p) => p,
                None => {
                    let due = deck.due_cards();
                    match due.first() {
                        Some(card) => card.problem.clone(),
                        None => {
                            println!("nothing due, all caught up");
                            return Ok(());
                        }
                    }
                }
            };

            deck::begin_review(&root, &problem)?;
            println!(
                "\x1b[33m⟳\x1b[0m reviewing '{}' — sol.cpp cleared (backup saved)",
                problem
            );
            println!("  solve it, then run `lcsrs good` or `lcsrs again`");
            open_editor(&root, &problem);
        }
        Cmd::Good => rate_active(&root, &mut deck, Rating::Good)?,
        Cmd::Again => rate_active(&root, &mut deck, Rating::Again)?,
        Cmd::Status => todo!(),
        Cmd::Import => todo!(),
    }

    Ok(())
}
