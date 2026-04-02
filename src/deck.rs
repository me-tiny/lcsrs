use crate::srs::{Card, Rating};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const DECK_FILE: &str = ".lcsrs.json";
const BACKUP_DIR: &str = ".lcsrs/backups";
const ACTIVE_FILE: &str = ".lcsrs/active";

#[derive(Serialize, Deserialize)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    /// resolve deck file path
    fn deck_path(root: &Path) -> PathBuf {
        todo!()
    }
    /// load deck if it exists
    pub fn load(root: &Path) -> anyhow::Result<Self> {
        todo!()
    }
    /// save deck to json file
    pub fn save(&self, root: &Path) -> anyhow::Result<()> {
        todo!()
    }
    /// find problem in deck
    pub fn find(&self, problem: &str) -> Option<usize> {
        todo!()
    }
    /// add card to deck if it doesn't exist already
    pub fn add(&mut self, problem: String) -> bool {
        todo!()
    }
    /// get list of due cards
    pub fn due_cards(&self) -> Vec<&Card> {
        todo!()
    }
    /// rate card and update it's interval
    pub fn rate(&mut self, problem: &str, rating: Rating) -> anyhow::Result<()> {
        todo!()
    }
}
