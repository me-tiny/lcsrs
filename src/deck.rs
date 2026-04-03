use crate::srs::{Card, Rating};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const DECK_FILE: &str = ".lcsrs.json";
const BACKUP_DIR: &str = ".lcsrs/backups";
const ACTIVE_FILE: &str = ".lcsrs/active";

#[derive(Default, Serialize, Deserialize)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    /// resolve deck file path
    fn deck_path(root: &Path) -> PathBuf {
        root.join(DECK_FILE)
    }
    /// load deck if it exists
    pub fn load(root: &Path) -> anyhow::Result<Self> {
        let path = Self::deck_path(root);
        if !path.exists() {
            return Ok(Self::default());
        }
        let data = std::fs::read_to_string(&path)?;
        let deck: Deck = serde_json::from_str(&data)?;
        Ok(deck)
    }
    /// save deck to json file
    pub fn save(&self, root: &Path) -> anyhow::Result<()> {
        let path = Self::deck_path(root);
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }
    /// find problem in deck
    pub fn find(&self, problem: &str) -> Option<usize> {
        self.cards.iter().position(|c| c.problem == problem)
    }
    /// add card to deck if it doesn't exist already
    pub fn add(&mut self, problem: String) -> bool {
        if self.find(&problem).is_some() {
            return false;
        }
        self.cards.push(Card::new(problem));
        true
    }
    /// get list of due cards
    pub fn due_cards(&self) -> Vec<&Card> {
        let mut due: Vec<&Card> = self.cards.iter().filter(|c| c.is_due()).collect();
        due.sort_by_key(|b| std::cmp::Reverse(b.days_overdue()));
        due
    }
    /// rate card and update it's interval
    pub fn rate(&mut self, problem: &str, rating: Rating) -> anyhow::Result<()> {
        let i = self
            .find(problem)
            .ok_or_else(|| anyhow::anyhow!("problem {} not in deck", problem))?;
        self.cards[i].review(rating);
        Ok(())
    }
}

/// reads which problem is currently being reviewed
pub fn active_review(root: &Path) -> anyhow::Result<Option<String>> {
    let active_path = root.join(ACTIVE_FILE);
    if !active_path.exists() {
        return Ok(None);
    }
    let problem = std::fs::read_to_string(&active_path)?.trim().to_string();
    Ok(Some(problem))
}

fn generate_template() -> String {
    r#"#include "lc.hpp"

class Solution {
  public:
    // TODO: paste method signature here
};

int main() {
    Solution s;

    // test cases
    // example:
    //
    // ordered:
    //
    // std::vector<int> t1{1, 2, 3, 1};
    // expect(s.containsDuplicate(t1), true);
    //
    // unordered:
    //
    // std::vector<int> n1 = {2, 7, 11, 15};
    // int t1 = 9;
    // std::vector<int> e1 = {0, 1};
    // expect_unordered(s.twoSum(n1, t1), e1);

    return 0;
}
"#
    .to_string()
}
