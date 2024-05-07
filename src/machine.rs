use crate::op::{Op, OpCode, Word};
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

    pub fn run(&mut self, debug: bool) -> Result<(), String> {
        while !self.halted {
            self.exeucte(debug)?;
        }
        Ok(())
    }

    fn exeucte(&mut self, debug: bool) -> Result<(), String> {
        if self.ip > self.program_size {
            return Err("segmentation fault".to_string());
        }

        let op = self.parse_op()?;
        if debug {
            println!(
                "[DEBUG] ip = {:0>3}  |  op = {: <10} |  stack = {}",
                self.ip,
                format!("{:?}", op),
                self.stack
            );
        }

        match op {
            Op::Push(word) => self.stack.push(word)?,
            Op::Pop => drop(self.stack.pop()?),
            Op::Echo => println!("{}", self.stack.pop()?),
            Op::Halt => self.halted = true,
            Op::Add => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a + b)?;
            }
            Op::Sub => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a - b)?;
            }
            Op::Mul => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a * b)?;
            }
            Op::Div => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a / b)?;
            }
        }

        Ok(())
    }

    fn parse_op(&mut self) -> Result<Op, String> {
        let code = self.program[self.ip];
        self.ip += 1;

        match code {
            OpCode::PUSH => Ok(Op::Push(self.extract_word()?)),
            OpCode::POP => Ok(Op::Pop),
            OpCode::ECHO => Ok(Op::Echo),
            OpCode::HALT => Ok(Op::Halt),
            OpCode::ADD => Ok(Op::Add),
            OpCode::SUB => Ok(Op::Sub),
            OpCode::DIV => Ok(Op::Div),
            OpCode::MUL => Ok(Op::Mul),
            _ => Err("unknown opcode encoutered".to_string()),
        }
    }

    fn extract_word(&mut self) -> Result<Word, String> {
        if self.ip + 1 > self.program_size {
            return Err("stack overflow".to_string());
        }
        let word = (self.program[self.ip] as Word) << 8 | self.program[self.ip + 1] as Word;
        self.ip += 2;
        Ok(word)
    }
}
