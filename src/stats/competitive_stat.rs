use std::collections::HashMap;

use crate::stats::utils::BeaverScore;
use crate::stats::utils::StatesAndSymbols;
pub struct CompetitiveStat {
    pub histograms: HashMap<StatesAndSymbols, Vec<Vec<usize>>>,
    pub leaderboards: HashMap<StatesAndSymbols, Vec<BeaverScore>>,
}

impl CompetitiveStat {
    pub const STAT_LIST_LENGTH: usize = 16;

    pub fn new() -> Self {
        CompetitiveStat {
            histograms: HashMap::new(),
            leaderboards: HashMap::new(),
        }
    }

    pub fn add(&mut self, states_and_symbols: StatesAndSymbols, beaver_score: BeaverScore) {
        let beaver_id = beaver_score.beaver;
        let score = beaver_score.score;

        let histogram = self.histograms
            .entry(states_and_symbols)
            .or_default();
    
        if histogram.len() <= score {
            histogram.resize(score + 1, Vec::new());
        }

        histogram
            .get_mut(score)
            .unwrap()
            .push(beaver_id);

        let leaderboard = self.leaderboards
            .entry(states_and_symbols)
            .or_default();

        let position = leaderboard.binary_search(&beaver_score).unwrap_or_else(|e| e);
        leaderboard.insert(position, beaver_score);
        leaderboard.resize(Self::STAT_LIST_LENGTH, BeaverScore::default());
    }
}


