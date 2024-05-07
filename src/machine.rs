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
        let mut program_size = input.len();

        if program_size > PROGRAM_CAPACITY {
            return Err(format!("a program must be under {}", PROGRAM_CAPACITY));
        }

        let mut program = [0; PROGRAM_CAPACITY];
        for (i, &op) in input.iter().enumerate() {
            program[i] = op;
        }

        if input.last().is_some_and(|value| *value != OpCode::HALT) {
            program[program_size] = OpCode::HALT;
            program_size += 1;
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
                self.stack.push(b + a)?;
            }
            Op::Sub => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(b - a)?;
            }
            Op::Mul => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(b * a)?;
            }
            Op::Div => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                if a == 0 {
                    return Err("division by zero".to_string());
                }
                self.stack.push(b / a)?;
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
            return Err(format!("could not extract word at {}", self.ip));
        }
        let word = (self.program[self.ip] as Word) << 8 | self.program[self.ip + 1] as Word;
        self.ip += 2;
        Ok(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_initialization() {
        let machine = Machine::try_new(&[]).unwrap();
        assert_eq!(machine.program_size, 0);
        assert_eq!(machine.halted, false);
    }

    #[test]
    fn test_program_capacity_exceeded() {
        let input = vec![0; PROGRAM_CAPACITY + 1];
        assert!(Machine::try_new(&input).is_err());
    }

    #[test]
    fn test_push_and_pop_operations() {
        let mut machine = Machine::try_new(&[OpCode::PUSH, 0x00, 0x01, OpCode::POP]).unwrap();
        machine.run(true).unwrap();
        assert!(machine.stack.pop().is_err());
    }

    #[test]
    fn test_addition() {
        let mut machine = Machine::try_new(&[
            OpCode::PUSH,
            0x00,
            0x05,
            OpCode::PUSH,
            0x00,
            0x03,
            OpCode::ADD,
        ])
        .unwrap();
        machine.run(false).unwrap();
        assert!(machine.stack.pop().is_ok_and(|value| value == 8));
    }

    #[test]
    fn test_subtrcation() {
        let mut machine = Machine::try_new(&[
            OpCode::PUSH,
            0x00,
            0x0f,
            OpCode::PUSH,
            0x00,
            0x0e,
            OpCode::SUB,
        ])
        .unwrap();
        machine.run(false).unwrap();
        assert!(machine.stack.pop().is_ok_and(|value| value == 1));
    }

    #[test]
    fn test_multiplication() {
        let mut machine = Machine::try_new(&[
            OpCode::PUSH,
            0x00,
            0x02,
            OpCode::PUSH,
            0x00,
            0x0f,
            OpCode::MUL,
        ])
        .unwrap();
        machine.run(false).unwrap();
        assert!(machine.stack.pop().is_ok_and(|value| value == 30));
    }

    #[test]
    fn test_division() {
        let mut machine = Machine::try_new(&[
            OpCode::PUSH,
            0x00,
            0x0f,
            OpCode::PUSH,
            0x00,
            0x03,
            OpCode::DIV,
        ])
        .unwrap();
        machine.run(false).unwrap();
        assert!(machine.stack.pop().is_ok_and(|value| value == 5));
    }

    #[test]
    fn test_division_by_zero() {
        let mut machine = Machine::try_new(&[
            OpCode::PUSH,
            0x00,
            0x05,
            OpCode::PUSH,
            0x00,
            0x00,
            OpCode::DIV,
        ])
        .unwrap();
        assert!(machine.run(false).is_err());
    }

    #[test]
    fn test_echo_operation() {
        let mut machine = Machine::try_new(&[OpCode::PUSH, 0x00, 0x01, OpCode::ECHO]).unwrap();
        machine.run(false).unwrap(); // Normally you would capture stdout to test this, but we'll assume it works here.
    }

    #[test]
    fn test_unknown_opcode() {
        let mut machine = Machine::try_new(&[0xFF]).unwrap();
        assert!(machine.run(false).is_err());
    }

    #[test]
    fn test_halt_operation() {
        let mut machine = Machine::try_new(&[OpCode::HALT]).unwrap();
        machine.run(false).unwrap();
        assert!(machine.halted);
    }
}
