use std::collections::BinaryHeap;
use std::collections::HashMap;

use crate::stats::utils::BeaverScore;
use crate::stats::utils::StatesAndSymbols;
pub struct CompetitiveStat {
    histograms: HashMap<StatesAndSymbols, Vec<Vec<usize>>>,
    leaderboards: HashMap<StatesAndSymbols, BinaryHeap<BeaverScore>>,
}

impl CompetitiveStat {
    const CONTAINER_SIZE: usize = 16;

    pub fn new() -> Self {
        CompetitiveStat {
            histograms: HashMap::new(),
            leaderboards: HashMap::new(),
        }
    }

    pub fn add(&mut self, states_and_symbols: StatesAndSymbols, beaver_score: BeaverScore) {
        let beaver_id = beaver_score.beaver;
        let score = beaver_score.score;

        let histogram = self.histograms.entry(states_and_symbols.clone()).or_insert(vec![Vec::new(); Self::CONTAINER_SIZE]);
        let leaderboard = self.leaderboards.entry(states_and_symbols.clone()).or_insert(BinaryHeap::new());

        match histogram.get_mut(score) {
            Some(beavers) => beavers.push(beaver_id),
            None => (),
        }

        leaderboard.push(beaver_score);
        if leaderboard.len() > Self::CONTAINER_SIZE {
            leaderboard.pop();
        }
    }
}