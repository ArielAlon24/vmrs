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

    pub fn head(&mut self) -> Result<Word, String> {
        if self.index == 0 {
            return Err("stack underflow".to_string());
        }
        Ok(self.buffer[self.index])
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
            return write!(f, "None");
        }
        write!(
            f,
            "{} -> None",
            self.buffer
                .iter()
                .map(|v| format!("{}", v))
                .take(self.index)
                .rev()
                .collect::<Vec<String>>()
                .join(" -> ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack() {
        let stack: Stack = Stack::new();
        assert_eq!(stack.index, 0);
    }

    #[test]
    fn test_push_and_pop() {
        let mut stack = Stack::new();
        assert!(stack.push(10).is_ok());
        assert_eq!(stack.pop().unwrap(), 10);
    }

    #[test]
    fn test_stack_overflow() {
        let mut stack = Stack::new();
        for _ in 0..STACK_CAPACITY {
            assert!(stack.push(10).is_ok());
        }
        assert!(stack.push(10).is_err());
    }

    #[test]
    fn test_stack_underflow() {
        let mut stack = Stack::new();
        assert!(stack.pop().is_err());
    }

    #[test]
    fn test_display_stack() {
        let mut stack = Stack::new();
        stack.push(10).unwrap();
        stack.push(20).unwrap();
        stack.push(30).unwrap();
        let display_format = format!("{}", stack);
        assert_eq!(display_format, "30 -> 20 -> 10 -> None");
    }

    #[test]
    fn test_display_empty_stack() {
        let mut stack = Stack::new();
        stack.push(10).unwrap();
        stack.push(20).unwrap();
        stack.push(30).unwrap();
        stack.pop().unwrap();
        stack.pop().unwrap();
        stack.pop().unwrap();
        let display_format = format!("{}", stack);
        assert_eq!(display_format, "None");
    }
}
