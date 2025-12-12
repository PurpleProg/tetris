use serde::{Deserialize, Serialize};

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
            username: context.username.clone(),
        }
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

impl ToString for Entry {
    fn to_string(&self) -> String {
        format!(
            "{: <20}: {: <10} level : {: >3}\n",
            self.username, self.score, self.level
        )
    }
}

#[derive(Debug)]
pub enum EntryError {
    EntryNotFount,
}

#[derive(Serialize, Deserialize)]
pub struct LeaderBoard {
    pub entrys: Vec<Entry>,
}
impl LeaderBoard {
    pub fn load(path: &str) -> Self {
        let string: String = std::fs::read_to_string(path).expect("cannot load save file");
        Self {
            entrys: serde_json::from_str(&string).expect("error serialising")
            // .unwrap_or(Vec::new()), // unwrap_or(vec![Entry {username: "failed to load save file".to_owned(), score: 42, level: 42,}])
        }
    }
    pub fn save(self, path: &str) {
        let string = serde_json::to_string(&self).expect("failed to serialize");
        println!("saving {}", string);
        println!("leaderboraed {}", self.to_string());
        std::fs::write(path, string).expect("failed to write to file");
    }
    pub fn add_entry(&mut self, entry: Entry) {
        self.entrys.push(entry);
    }
    pub fn get_entry(&mut self, username: &str) -> Option<&mut Entry> {
        for entry in self.entrys.iter_mut() {
            if entry.username == username {
                return Some(entry);
            }
        }
        None
    }
    pub fn update_entry(
        &mut self,
        username: &str,
        score: u64,
        level: u64,
    ) -> Result<(), EntryError> {
        let entry: &mut Entry = self.get_entry(username).ok_or(EntryError::EntryNotFount)?;
        if score > entry.score {
            entry.level = level;
            entry.score = score;
        }
        println!("updated entry: {}", entry.to_string());
        Ok(())
    }
}

impl ToString for LeaderBoard {
    fn to_string(&self) -> String {
        self.entrys
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .concat()
    }
}
