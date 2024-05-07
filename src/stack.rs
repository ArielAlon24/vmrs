use std::fmt;

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

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.index == 0 {
            write!(f, "None")
        } else {
            write!(
                f,
                "{} -> None",
                self.buffer
                    .iter()
                    .map(|v| format!("{}", v))
                    .take(self.index)
                    .collect::<Vec<String>>()
                    .join(" -> ")
            )
        }
    }
}
