use std::collections::HashMap;

use crate::fast_queue::FastQueue;

struct Beaver {
    id: u64,
}

struct BusyBeavers<'a> {
    running: Vec<FastQueue<'a, u64>>, // running[n] = queue of running n-state beavers
    halting_hg: Vec<Vec<Vec<u64>>>, // halting_hg[n][m] = list of n-state beavers that halt after m steps (histogram)
    halting_lb: Vec<HashMap<u64, u64>>, // halting_lb[]
    dormant: Vec<u64>,
    propagating: Vec<u64>,
    
}


impl<'a> BusyBeavers<'a> {
    fn create_beaver(id: u64) {
        
    }
}