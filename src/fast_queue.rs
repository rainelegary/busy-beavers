
pub struct FastQueue<T> {
    stacks: (Vec<T>, Vec<T>),
    push_stack: u8,
    pop_index: usize,
}

impl<T: Clone> FastQueue<T> {
    pub fn new() -> Self {
        FastQueue {
            stacks: (Vec::new(), Vec::new()),
            push_stack: 0,
            pop_index: 0,
        }
    }

    pub fn from(array: &[T]) -> Self {
        let mut queue = FastQueue::new();
        for item in array {
            queue.push(item.clone());
        }
        queue
    }

    pub fn push(&mut self, value: T) {
        let previously_empty = self.stacks.0.len() + self.stacks.1.len() == 0;

        match self.push_stack {
            0 => self.stacks.0.push(value),
            1 => self.stacks.1.push(value),
            _ => panic!(),
        }

        if previously_empty {
            // make the queue popppable
            self.push_stack = (self.push_stack + 1) % 2;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let pop_stack = match self.push_stack {
            0 => &mut self.stacks.1,
            1 => &mut self.stacks.0,
            _ => panic!(),
        };
        let value = pop_stack.get(self.pop_index).cloned();
        self.pop_index += 1;
        if self.pop_index == pop_stack.len() {
            pop_stack.clear();
            self.push_stack = (self.push_stack + 1) % 2;
            self.pop_index = 0;
        }
        value
    }
}