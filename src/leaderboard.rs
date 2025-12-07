#[derive(Debug)]
pub struct Entry {
    pub level: u64,
    pub score: u64,
    pub username: String,
}

impl Entry {
    pub fn new(level: u64, score: u64, username: String) -> Self {
        Entry {
            level,
            score,
            username,
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
