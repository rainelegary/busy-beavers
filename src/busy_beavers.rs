use std::collections::HashMap;

use crate::fast_queue::FastQueue;
use crate::beaver_stats::BeaverStats;
use crate::turing_machine::TuringMachine;
use crate::turing_machine::TFn;

struct BusyBeavers {
    beavers: HashMap<u64, TuringMachine>,
    running: Vec<FastQueue<u64>>,
    stats: BeaverStats,
    run_number: u64,
    least_running_states: usize,
    most_running_states: usize,
}


impl<'a> BusyBeavers {
    const RUN_DURATION: u64 = 10;

    pub fn new() -> Self {
        let beavers = HashMap::from([
            (0, TuringMachine::new(TFn::new()))
        ]);

        let mut busy_beavers = BusyBeavers {
            beavers: beavers,
            running: vec![FastQueue::from(&[0])],
            stats: BeaverStats::new(),
            run_number: 0,
            least_running_states: 1,
            most_running_states: 1,
        };
        busy_beavers.running[0].push(0);
        busy_beavers
    }

    fn run(&mut self) {

    } 
}