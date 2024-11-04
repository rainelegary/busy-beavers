use itertools::iproduct;

use std::cmp::max;
use std::collections::HashMap;

use crate::fast_queue::FastQueue;
use crate::stats::beaver_stats::BeaverStats;
use crate::stats::utils::StatesAndSymbols;
use crate::turing_machine::TFnValue;
use crate::turing_machine::TuringMachine;
use crate::stats::utils::BeaverType;

pub struct BusyBeavers {
    pub beavers: Vec<TuringMachine>,
    running: HashMap<StatesAndSymbols, FastQueue<usize>>,
    stats: BeaverStats,
}


impl BusyBeavers {
    const RUN_DURATION: usize = 100;
    pub const MAX_DELTA: u8 = 1; 

    pub fn new() -> Self {
        BusyBeavers {
            beavers: vec![TuringMachine::new(HashMap::new())],
            running: HashMap::from([(
                StatesAndSymbols { n_states: 1, n_symbols: 1 },
                FastQueue::from(&[0]),
            )]),
            stats: BeaverStats::new(),
        }
    }

    pub fn run(&mut self) {
        for run_number in 1_usize.. {
            let three_adic = (0_u8..).find(|x| run_number % 3_usize.pow(*x as u32) != 0).unwrap() - 1;
            let two_adic = (0_u8..).find(|x| run_number % 2_usize.pow(*x as u32) != 0).unwrap() - 1;

            let n_states = three_adic + 1;
            let n_symbols = two_adic + 1;
            let states_and_symbols = StatesAndSymbols { n_states, n_symbols };

            let Some(fast_queue) = self.running.get_mut(&states_and_symbols) else { continue };
            let Some(beaver_id) = fast_queue.pop() else { continue };

            let beaver = &mut self.beavers[beaver_id];
            beaver.run(Self::RUN_DURATION);
            
            let class = self.update_stats_and_classify(beaver_id);
                
            if class == BeaverType::Undetermined {
                self.running.get_mut(&states_and_symbols).unwrap().push(beaver_id);
            }

            if class == BeaverType::Halting {
                self.proliferate(beaver_id);
            }
        }
    }

    fn update_stats_and_classify(&mut self, beaver_id: usize) -> BeaverType {
        let beaver = &mut self.beavers[beaver_id];
        if beaver.halted {
            self.stats.update_halting_stats(beaver_id, beaver);
            self.stats.subtract_undetermined(beaver.states_and_symbols);
            return BeaverType::Halting;
        }
        
        let history_len = beaver.history.len();
        let prev_history_len = history_len - Self::RUN_DURATION;
        if history_len.leading_zeros() == prev_history_len.leading_zeros() {
            // number of steps hasn't reached a new power of two, so to
            // save computational cost we will not run the loop test 
            return BeaverType::Undetermined;
        }

        if let Some(periodicity) = self.loop_test(beaver_id) {
            let beaver = &mut self.beavers[beaver_id];
            self.stats.update_infinite_stats(beaver_id, beaver, periodicity);
            self.stats.subtract_undetermined(beaver.states_and_symbols);
            return BeaverType::Infinite;
        }

        BeaverType::Undetermined
    }

    fn proliferate(&mut self, beaver_id: usize) {
        let beaver = self.beavers[beaver_id].clone();

        let states_range = 0..beaver.states_and_symbols.n_states + 1;
        let symbols_range = 0..beaver.states_and_symbols.n_symbols + 1;
        let deltas_range = match beaver.t_fn.len() {
            0 => 0..=Self::MAX_DELTA as i8,
            _ => -(Self::MAX_DELTA as i8)..=Self::MAX_DELTA as i8,
        };

        let current_status = beaver.history.last().unwrap();
        for (state, symbol, delta) in iproduct!(states_range, symbols_range, deltas_range) {
            let mut child = beaver.clone();
            child.t_fn.insert(current_status.clone(), TFnValue { state, symbol, delta });
            child.halted = false;
            child.states_and_symbols = StatesAndSymbols {
                n_states: max(beaver.states_and_symbols.n_states, state + 1),
                n_symbols: max(beaver.states_and_symbols.n_symbols, symbol + 1),
            };

            let child_id = self.beavers.len();
            self.running
                .entry(child.states_and_symbols)
                .or_default()
                .push(child_id);
            self.stats.add_undetermined(child.states_and_symbols);
            self.beavers.push(child);
        }
    }
}