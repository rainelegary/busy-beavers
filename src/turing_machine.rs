mod turing_machine_ui;

use std::collections::HashMap;

type TFn = HashMap<(u8, u8), (u8, u8, i8)>;

struct TuringMachine {
    t_fn: TFn,
    tape: (Vec<u8>, Vec<u8>),
    state: u8,
    head: isize,
    halted: bool,
}

impl TuringMachine {
    fn new(t_fn: TFn) -> Self {
        TuringMachine {
            t_fn,
            tape: (vec![], vec![0]),
            state: 0,
            head: 0,
            halted: false,
        }
    }
    
    fn vec_and_index(&mut self, loc: &isize) -> (&mut Vec<u8>, usize) {
        match loc {
            h if *h < 0 => (&mut self.tape.0, (-self.head - 1) as usize),
            _ => (&mut self.tape.1, self.head as usize),
        }
    }

    fn vec_and_index_head(&mut self) -> (&mut Vec<u8>, usize) {
        self.vec_and_index(&self.head.clone())
    }
    
    fn get_symbol(&mut self) -> u8 {
        let (&mut ref mut vec, index) = self.vec_and_index_head();
        vec[index]
    }

    fn set_symbol(&mut self, symbol: u8) { 
        let (&mut ref mut vec, index) = self.vec_and_index_head();
        vec[index] = symbol;
    }

    fn move_head(&mut self, delta: i8) {
        self.head += delta as isize;
        let (&mut ref mut vec, index) = self.vec_and_index_head();
        if index >= vec.len() {
            vec.resize(index + 1, 0);
        }
    }
    
    fn step(&mut self) {
        let symbol = self.get_symbol();
        if !self.t_fn.contains_key(&(self.state, symbol)) {
            println!("halt");
            self.halted = true;
            return;
        }
        let (state, symbol, delta) = self.t_fn[&(self.state, symbol)];
        self.state = state;
        self.set_symbol(symbol);
        self.move_head(delta);
    }

    fn run(&mut self, steps: u64) {
        for _ in 0..steps {
            self.show();
            self.step();
            if self.halted {
                break;
            }
        }
    }
}


fn main() {
    let t_fn = TFn::from([
        ((0, 0), (1, 1, 1)),
        ((0, 1), (1, 1, -1)),
        ((1, 0), (0, 1, -1)),
    ]);

    let mut tm = TuringMachine::new(t_fn);

    tm.run(10);
}