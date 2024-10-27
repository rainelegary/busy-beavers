mod turing_machine;
mod turing_machine_ui;
mod busy_beavers;
mod fast_queue;
mod stats;
mod loop_test;

use crate::busy_beavers::BusyBeavers;

fn main() {
    let mut busy_beavers = BusyBeavers::new();
    busy_beavers.run();

    // let t_fn: TFn = TFn::from([
    //     ((0, 0), (1, 1, 1)),
    //     ((0, 1), (1, 1, -1)),
    //     ((1, 0), (0, 1, -1)),
    // ]);
    
    // let mut tm = TuringMachine::new(t_fn);
    
    // tm.run(10);
}

