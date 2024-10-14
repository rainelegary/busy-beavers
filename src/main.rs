mod turing_machine;
mod turing_machine_ui;
mod busy_beavers;
mod fast_queue;
mod beaver_stats;
mod enums;
mod loop_test;

use crate::turing_machine::TFn; 
use crate::turing_machine::TuringMachine;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let history = vec![0, 1, 2, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
    let periodicity = 2;

    let pc_lifetime = (0..history.len()).rev().skip(periodicity).find(
        |&i| history[i] != history[i + periodicity]
    ).unwrap();

    println!("{}", pc_lifetime);

    // let t_fn: TFn = TFn::from([
    //     ((0, 0), (1, 1, 1)),
    //     ((0, 1), (1, 1, -1)),
    //     ((1, 0), (0, 1, -1)),
    // ]);
    
    // let mut tm = TuringMachine::new(t_fn);
    
    // tm.run(10);
}

