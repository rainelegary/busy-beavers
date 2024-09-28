use std::collections::HashMap;

pub type TFn = HashMap<(u8, u8), (u8, u8, i8)>;

pub struct TuringMachine {
    pub t_fn: TFn,
    pub tape: (Vec<u8>, Vec<u8>),
    pub state: u8,
    pub head: isize,
    pub halted: bool,
}

impl TuringMachine {
    pub fn new(t_fn: TFn) -> Self {
        TuringMachine {
            t_fn,
            tape: (vec![], vec![0]),
            state: 0,
            head: 0,
            halted: false,
        }
    }
    
    pub fn vec_and_index(&mut self, loc: &isize) -> (&mut Vec<u8>, usize) {
        match *loc {
            l if l < 0 => (&mut self.tape.0, (-l - 1) as usize),
            l => (&mut self.tape.1, l as usize),
        }
    }

    fn vec_and_index_head(&mut self) -> (&mut Vec<u8>, usize) {
        self.vec_and_index(&self.head.clone())
    }
    
    pub fn get_symbol(&mut self) -> u8 {
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
            self.halted = true;
            return;
        }
        let (state, symbol, delta) = self.t_fn[&(self.state, symbol)];
        self.state = state;
        self.set_symbol(symbol);
        self.move_head(delta);
    }

    pub fn run(&mut self, steps: u64) {
        for _ in 0..steps {
            self.show_tape_and_state();
            self.step();
            if self.halted {
                break;
            }
        }
    }
}