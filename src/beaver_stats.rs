use strum::IntoEnumIterator;

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;

use crate::enums::BeaverType;
use crate::enums::HaltingStats;
use crate::enums::InfiniteStats;

type Histogram = Vec<Vec<Vec<u64>>>; // histogram[n][m] = list of n-state beavers with a score of m
type Leaderboard = Vec<BinaryHeap<BeaverScore>>; // leaderboard[n] = max-heap of n-state beavers with highest scores

#[derive(Eq, PartialEq)]
struct BeaverScore {
    beaver: u64,
    score: u64,
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
 
struct AggregateStat {
    tally: u64,
    tallies: Vec<u64>,
}

impl AggregateStat {
    fn new() -> Self {
        AggregateStat {
            tally: 0,
            tallies: Vec::new(),
        }
    }
}

struct CompetitiveStat {
    histogram: Histogram,
    leaderboard: Leaderboard,
}

impl CompetitiveStat {
    fn new() -> Self {
        CompetitiveStat {
            histogram: Vec::new(),
            leaderboard: Vec::new(),
        }
    }
}

pub struct BeaverStats {
    types: HashMap<BeaverType, AggregateStat>,
    halting: HashMap<HaltingStats, CompetitiveStat>,
    infinite: HashMap<InfiniteStats, CompetitiveStat>,
}

impl BeaverStats {
    pub fn new() -> Self {
        BeaverStats { 
            types: BeaverType::iter().map(
                |beaver_type|
                (beaver_type, AggregateStat::new())
            ).collect(),

            halting: HaltingStats::iter().map(
                |stat|
                (stat, CompetitiveStat::new())
            ).collect(),

            infinite: InfiniteStats::iter().map(
                |stat|
                (stat, CompetitiveStat::new())
            ).collect()
        }
    }
}