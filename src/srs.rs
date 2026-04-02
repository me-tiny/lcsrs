use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

// SuperMemo 2
// https://www.supermemo.com/en/blog/application-of-a-computer-to-improve-the-results-obtained-in-working-with-the-supermemo-method
// the algorithm used is inspired by SuperMemo 2
// EF' = EF + (0.1-(5-q)*(0.08+(5-q)*0.02))
// where
// EF' = new value of e-factor
// EF = old value of e-factor
// q = difficult of response in 0-5 scale
// modified in my case to only use Good/Again, and custom values for better fine tuning this more
// closely represents the algorithm that was used by Anki before the switch to FSRS
// NOTE: may switch to FSRS depending on how this implementation goes

const INITIAL_INTERVAL_DAYS: f64 = 1.0;
const INITIAL_EASE: f64 = 2.5;
const MIN_EASE: f64 = 1.5;
const EASE_BONUS_GOOD: f64 = 0.1;
const EASE_PENALTY_AGAIN: f64 = 0.3;

/// rating is calculated via constants EASE_BONUS_GOOD and EASE_PENALTY_AGAIN
/// when rated Good, will add EASE_BONUS_GOOD to current ease, and get the minimum between current
/// ease and 3.5
/// when rated Again, will minus EASE_PENALTY_AGAIN from current ease, and get the max between
/// current ease and MIN_EASE
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Rating {
    Good,
    Again,
}

#[derive(Serialize, Deserialize)]
pub struct Card {
    /// problem directory name, e.g. "0001-two-sum"
    pub problem: String,
    /// current interval in days
    pub interval: f64,
    /// ease factor
    pub ease: f64,
    /// next review date
    pub due: NaiveDate,
    /// total number of reviews
    pub reviews: u32,
    /// consecutive "Good" streak, resetting on "Again"
    pub streak: u32,
}
