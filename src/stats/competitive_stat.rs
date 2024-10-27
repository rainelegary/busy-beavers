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

        self.histograms
            .entry(states_and_symbols)
            .or_insert(vec![Vec::new(); Self::CONTAINER_SIZE])
            .get_mut(score)
            .unwrap()
            .push(beaver_id);

        let leaderboard = self.leaderboards
            .entry(states_and_symbols)
            .or_default();

        leaderboard.push(beaver_score);
        if leaderboard.len() > Self::CONTAINER_SIZE {
            leaderboard.pop();
        }
    }
}