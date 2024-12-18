mod turing_machine;
mod turing_machine_ui;
mod busy_beavers;
mod fast_queue;
mod stats;
mod loop_test;

use stats::utils::{BeaverType, InfiniteStat, Stat};

use crate::busy_beavers::BusyBeavers;

fn main() {
    
    let beaver_type = Stat::Infinite(InfiniteStat::PCLifetime);
    let beaver_type_str = beaver_type.to_string();
    println!("Enum to String: {}", beaver_type_str); // Outputs: "halting"
    
    let parsed_beaver_type: Stat = "pc_lifetime".parse().unwrap();
    println!("String to Enum: {:?}", parsed_beaver_type.to_string()); // Outputs: "Halting"
    
    let mut busy_beavers = BusyBeavers::new();

    // busy_beavers.run();


    // let t_fn: TFn = TFn::from([
    //     ((0, 0), (1, 1, 1)),
    //     ((0, 1), (1, 1, -1)),
    //     ((1, 0), (0, 1, -1)),
    // ]);
    
    // let mut tm = TuringMachine::new(t_fn);
    
    // tm.run(10);
}

