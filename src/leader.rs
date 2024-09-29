use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
struct Leader {
    id: u64,
    score: u64,
}

impl Ord for Leader {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Leader {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}