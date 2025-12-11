use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};

use crate::GameContext;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub level: u64,
    pub score: u64,
    pub username: String,
}

impl Entry {
    pub fn new(context: &GameContext) -> Self {
        Entry {
            level: context.level,
            score: context.score,
            username: "implement me".to_owned(), // TODO: get $USER
        }
    }
    fn empty() -> Self {
        Self { level: 0, score: 0, username: "TEST_USERNAME".to_owned()}
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        return self.score == other.score;
    }
    fn ne(&self, other: &Self) -> bool {
        return self.score != other.score;
    }
}

impl PartialOrd for Entry {
    fn ge(&self, other: &Self) -> bool {
        return self.score >= other.score;
    }
    fn gt(&self, other: &Self) -> bool {
        return self.score > other.score;
    }
    fn le(&self, other: &Self) -> bool {
        return self.score <= other.score;
    }
    fn lt(&self, other: &Self) -> bool {
        return self.score < other.score;
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self > other {
            return Some(std::cmp::Ordering::Greater);
        }
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }
        if self < other {
            return Some(std::cmp::Ordering::Less);
        }
        return None;
    }
}

type LeaderBoard = Vec<Entry>;

pub fn save_score(context: &GameContext) {
    let entry = Entry::new(context);
    write!(
        File::create(".scores").expect("cannot create file !"),
        "{}",
        serde_json::to_string(&entry).expect("failed to serialize")
    )
    .expect("failed to write to file");
}

pub fn load_leaderboard() -> LeaderBoard {
    vec![Entry::empty()]

}
