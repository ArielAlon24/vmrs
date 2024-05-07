use crate::op::Word;

const STACK_CAPACITY: usize = 1 << 10;

pub struct Stack {
    buffer: [Word; STACK_CAPACITY],
    index: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            buffer: [0; STACK_CAPACITY],
            index: 0,
        }
    }

    pub fn push(&mut self, word: Word) -> Result<(), String> {
        if self.index >= STACK_CAPACITY {
            return Err("stack overflow".to_string());
        }

        self.buffer[self.index] = word;
        self.index += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Word, String> {
        if self.index == 0 {
            return Err("stack underflow".to_string());
        }
        self.index -= 1;
        Ok(self.buffer[self.index])
    }
}
