use std::collections::HashMap;

use crate::fast_queue::FastQueue;
use crate::beaver_stats::BeaverStats;
use crate::turing_machine::TuringMachine;
use crate::turing_machine::TFn;
use crate::enums::BeaverType;

pub struct BusyBeavers {
    beavers: HashMap<u64, TuringMachine>,
    running: Vec<FastQueue<u64>>,
    stats: BeaverStats,
    run_number: u64,
}


impl BusyBeavers {
    const RUN_DURATION: usize = 100;

    pub fn new() -> Self {
        BusyBeavers {
            beavers: HashMap::from([(0, TuringMachine::new(TFn::new()))]),
            running: vec![FastQueue::new(), FastQueue::from(&[0])],
            stats: BeaverStats::new(),
            run_number: 1,
        }
    }

    fn run(&mut self) {
        let mut lowest_states_running = 1;
        for run_number in 1_usize.. {
            let num_states = (run_number.trailing_zeros() as usize) + lowest_states_running;

            let Some(fast_queue) = self.running.get_mut(num_states) else { continue };
            let Some(beaver_id) = fast_queue.pop() else { continue };

            let beaver = self.beavers.get_mut(&beaver_id).unwrap();
            beaver.run(Self::RUN_DURATION);

            // classify beaver

            // add beaver back into running if undetermined
            
            // proliferate beaver if halting
            // - add children to beavers
            // - add children to running
            // expand the running vector if necessary
        }
    }

    fn classify(&self, beaver: &mut TuringMachine) -> BeaverType {
        if beaver.halted {
            return BeaverType::Halting;
        }
        
        let history_len = beaver.history.len();
        let prev_history_len = history_len - Self::RUN_DURATION;
        if history_len.leading_zeros() == prev_history_len.leading_zeros() {
            // number of steps hasn't reached a new power of two, so we
            // will not run the loop test to save computational cost
            return BeaverType::Undetermined;
        }

        match self.loop_test(beaver) {
            Some(usize) => BeaverType::Infinite,
            None => BeaverType::Undetermined,
        }
    }
}