use std::collections::HashMap;

pub type TFn = HashMap<TFnKey, TFnValue>;

pub struct TuringMachine {
    pub t_fn: TFn,
    pub tape: Tape,
    pub head: isize,
    pub halted: bool,
    pub history: Vec<TFnKey>,
}

impl TuringMachine {
    pub fn new(t_fn: TFn) -> Self {
        TuringMachine {
            t_fn,
            tape: Tape::new(),
            head: 0,
            halted: false,
            history: vec![TFnKey{state: 0, symbol: 0}],
        }
    }
    
    pub fn run(&mut self, duration: usize) {
        for _ in 0..duration {
            self.show_tape_and_state();
            self.step();
            if self.halted {
                break;
            }
        }
    }
        
    fn step(&mut self) {
        let current = &self.history[self.history.len() - 1];
        if !self.t_fn.contains_key(current) {
            self.halted = true;
            return;
        }
        let next = self.t_fn[current].clone();
        self.set_symbol(next.symbol);
        self.move_head(next.delta);
    
        let local_symbol = self.get_symbol();
        self.history.push(TFnKey{state: next.state, symbol: local_symbol});
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
    
    fn vec_and_index_head(&mut self) -> (&mut Vec<u8>, usize) {
        self.vec_and_index(&self.head.clone())
    }
    
    pub fn vec_and_index(&mut self, loc: &isize) -> (&mut Vec<u8>, usize) {
        match *loc {
            l if l < 0 => (&mut self.tape.negative, (-l - 1) as usize),
            l => (&mut self.tape.positive, l as usize),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct TFnKey {
    pub state: u8,
    pub symbol: u8,
}

#[derive(Clone)]
pub struct TFnValue {
    state: u8,
    symbol: u8,
    pub delta: i8,
}

pub struct Tape {
    negative: Vec<u8>,
    positive: Vec<u8>,
}

impl Tape {
    fn new() -> Self {
        Tape {
            negative: vec![],
            positive: vec![0],
        }
    }

    pub fn len(&self) -> usize {
        self.negative.len() + self.positive.len()
    }


    pub fn footprint(&self) -> usize {
        self.negative.iter().map(|&x| x as usize).sum::<usize>() +
        self.positive.iter().map(|&x| x as usize).sum::<usize>()
    }
}