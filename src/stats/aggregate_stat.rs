use std::collections::HashMap;

use crate::stats::utils::StatesAndSymbols;

pub struct AggregateStat {
    pub tallies: HashMap<StatesAndSymbols, u64>,
}

impl AggregateStat {
    pub fn new() -> Self {
        AggregateStat {
            tallies: HashMap::new(),
        }
    }

    pub fn add(&mut self, states_and_symbols: StatesAndSymbols) {
        *self.tallies.entry(states_and_symbols).or_insert(0) += 1;
    }

    pub fn subtract(&mut self, states_and_symbols: StatesAndSymbols) {
        *self.tallies.get_mut(&states_and_symbols).unwrap() -= 1;
    }
}