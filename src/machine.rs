use crate::op::{OpType, Word};
use crate::stack::Stack;

const PROGRAM_CAPACITY: usize = 1 << 10;

pub struct Machine {
    stack: Stack,
    program: [u8; PROGRAM_CAPACITY],
    program_size: usize,
    halted: bool,
    ip: usize,
}

impl Machine {
    pub fn try_new(input: &[u8]) -> Result<Self, String> {
        let program_size = input.len();

        if program_size > PROGRAM_CAPACITY {
            return Err(format!("a program must be under {}", PROGRAM_CAPACITY));
        }

        let mut program = [0; PROGRAM_CAPACITY];
        for (i, &op) in input.iter().enumerate() {
            program[i] = op;
        }

        Ok(Self {
            stack: Stack::new(),
            program,
            program_size,
            ip: 0,
            halted: false,
        })
    }

    pub fn execute(&mut self) -> Result<(), String> {
        while !self.halted {
            self.exeucte_op()?;
        }
        Ok(())
    }

    fn extract_word(&mut self) -> Result<Word, String> {
        if self.ip + 1 > self.program_size {
            return Err("stack overflow".to_string());
        }
        let word = (self.program[self.ip] as Word) << 8 | self.program[self.ip + 1] as Word;
        self.ip += 2;
        Ok(word)
    }

    fn exeucte_op(&mut self) -> Result<(), String> {
        if self.ip > self.program_size {
            return Err("segmentation fault".to_string());
        }

        let op: OpType = self.program[self.ip].try_into()?;
        self.ip += 1;

        match op {
            OpType::Push => {
                let word = self.extract_word()?;
                self.stack.push(word)?;
            }
            OpType::Pop => drop(self.stack.pop()?),
            OpType::Echo => println!("{}", self.stack.pop()?),
            OpType::Halt => self.halted = true,
        }

        Ok(())
    }
}
