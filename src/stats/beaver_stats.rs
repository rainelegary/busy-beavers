use strum::IntoEnumIterator;

use std::collections::HashMap;

use crate::stats::aggregate_stat::AggregateStat;
use crate::stats::utils::BeaverScore;
use crate::stats::competitive_stat::CompetitiveStat;
use crate::stats::enums::BeaverType;
use crate::stats::enums::HaltingStats;
use crate::stats::enums::InfiniteStats;

use crate::turing_machine::TuringMachine;

use super::utils::StatesAndSymbols;

pub struct BeaverStats {
    types: HashMap<BeaverType, AggregateStat>,
    halting: HashMap<HaltingStats, CompetitiveStat>,
    infinite: HashMap<InfiniteStats, CompetitiveStat>,
}

impl BeaverStats {
    pub fn new() -> Self {
        let mut stats = BeaverStats { 
            types: BeaverType::iter().map(
                |beaver_type| (beaver_type, AggregateStat::new())
            ).collect(),

            halting: HaltingStats::iter().map(
                |stat| (stat, CompetitiveStat::new())
            ).collect(),

            infinite: InfiniteStats::iter().map(
                |stat| (stat, CompetitiveStat::new())
            ).collect(),
        };
        stats.add_undetermined(StatesAndSymbols { n_states: 1, n_symbols: 1 });
        stats
    }
    
    pub fn update_halting_stats(&mut self, beaver_id: usize, beaver: &mut TuringMachine) {
        let states_and_symbols = beaver.states_and_symbols;
        self.types.get_mut(&BeaverType::Halting).map(|stat| stat.add(states_and_symbols));
        
        let lifetime = BeaverScore { beaver: beaver_id, score: beaver.history.len() };
        let coverage = BeaverScore { beaver: beaver_id, score: beaver.tape.len() }; 
        let footprint = BeaverScore { beaver: beaver_id, score: beaver.tape.footprint() }; 
        
        self.halting.get_mut(&HaltingStats::Lifetime).map(|stat| stat.add(states_and_symbols, lifetime));
        self.halting.get_mut(&HaltingStats::Coverage).map(|stat| stat.add(states_and_symbols, coverage));
        self.halting.get_mut(&HaltingStats::Footprint).map(|stat| stat.add(states_and_symbols, footprint));
    }

    pub fn update_infinite_stats(&mut self, beaver_id: usize, beaver: &mut TuringMachine, periodicity: usize) {
        let states_and_symbols = beaver.states_and_symbols;
        let pc_lifetime = match (0..beaver.history.len()).rev().skip(periodicity).find(
            |&i| beaver.history[i] != beaver.history[i + periodicity]
        ) {
            Some(i) => i + 1,
            None => 0,
        };

        let mut beaver_copy = TuringMachine::new(beaver.t_fn.clone());
        beaver_copy.run(pc_lifetime);
        let head = beaver_copy.head;
        beaver_copy.run(periodicity);
        
        let periodicity = BeaverScore { beaver: beaver_id, score: periodicity };
        let loop_delta = BeaverScore { beaver: beaver_id, score: (beaver_copy.head - head).abs() as usize };
        let pc_lifetime = BeaverScore { beaver: beaver_id, score: pc_lifetime };
        let pc_coverage = BeaverScore { beaver: beaver_id, score: beaver_copy.tape.len() };
        let pc_footprint = BeaverScore { beaver: beaver_id, score: beaver_copy.tape.footprint() };

        self.infinite.get_mut(&InfiniteStats::Periodicity).map(|stat| stat.add(states_and_symbols, periodicity));
        self.infinite.get_mut(&InfiniteStats::LoopDelta).map(|stat| stat.add(states_and_symbols, loop_delta));
        self.infinite.get_mut(&InfiniteStats::PCLifetime).map(|stat| stat.add(states_and_symbols, pc_lifetime));
        self.infinite.get_mut(&InfiniteStats::PCCoverage).map(|stat| stat.add(states_and_symbols, pc_coverage));
        self.infinite.get_mut(&InfiniteStats::PCFootprint).map(|stat| stat.add(states_and_symbols, pc_footprint));
    }

    pub fn add_undetermined(&mut self, states_and_symbols: StatesAndSymbols) {
        self.types.get_mut(&BeaverType::Undetermined).map(|stat| stat.add(states_and_symbols));
    }

    pub fn subtract_undetermined(&mut self, states_and_symbols: StatesAndSymbols) {
        self.types.get_mut(&BeaverType::Undetermined).map(|stat| stat.subtract(states_and_symbols));
    }
}


 


