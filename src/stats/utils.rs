use std::cmp::Ordering;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct StatesAndSymbols {
    pub n_states: u8,
    pub n_symbols: u8,
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct BeaverScore {
    pub beaver: usize,
    pub score: usize,
}

impl Ord for BeaverScore {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

impl PartialOrd for BeaverScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}