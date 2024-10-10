mod turing_machine;
mod turing_machine_ui;
mod busy_beavers;
mod fast_queue;

use crate::turing_machine::TFn; 
use crate::turing_machine::TuringMachine;

fn main() {    
    let t_fn: TFn = TFn::from([
        ((0, 0), (1, 1, 1)),
        ((0, 1), (1, 1, -1)),
        ((1, 0), (0, 1, -1)),
    ]);
    
    let mut tm = TuringMachine::new(t_fn);
    
    tm.run(10);


    for p in 1..10 {
        for n in 1..p {
            let mut v = Vec::new();
            for i in 1..p {
                v.push((i * n) % p);
            }
            let m = v.iter().max().unwrap();
            let r = p - m;
            println!("f({}, {}) = {}", n, p, r); 
        }
    }
}

