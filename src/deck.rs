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
