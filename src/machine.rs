use crate::op::{Op, OpKind, Word};
use crate::stack::Stack;
use std::mem::size_of;

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

        if input
            .last()
            .is_some_and(|value| *value != OpKind::Halt.into())
        {
            program[program_size] = OpKind::Halt.into();
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
                "[DEBUG] {:0>3} | {: <20} | stack = {}",
                self.ip,
                format!("{:?}", op),
                self.stack
            );
        }

        match op {
            Op(OpKind::Push, Some(word)) => self.stack.push(word)?,
            Op(OpKind::Pop, None) => drop(self.stack.pop()?),
            Op(OpKind::Echo, None) => println!("{}", self.stack.head()?),
            Op(OpKind::Add, None) => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(b + a)?;
            }
            Op(OpKind::Sub, None) => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(b - a)?;
            }
            Op(OpKind::Mul, None) => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(b * a)?;
            }
            Op(OpKind::Div, None) => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                if a == 0 {
                    return Err("division by zero".to_string());
                }
                self.stack.push(b / a)?;
            }
            Op(OpKind::Goto, Some(value)) => {
                let address = usize::try_from(value).map_err(|_| "invalid address".to_string())?;
                if address > self.program_size {
                    return Err("segmentation fault".to_string());
                }
                self.ip = address;
            }
            Op(OpKind::Goif, Some(value)) => match self.stack.pop()? {
                0 => {}
                _ => {
                    let address =
                        usize::try_from(value).map_err(|_| "invalid address".to_string())?;
                    if address > self.program_size {
                        return Err("segmentation fault".to_string());
                    }
                    self.ip = address;
                }
            },
            Op(OpKind::Copy, None) => {
                let head = self.stack.head()?;
                self.stack.push(head)?;
            }
            Op(OpKind::Halt, None) => self.halted = true,
            _ => return Err("incorrect op code encountered".to_string()),
        }

        Ok(())
    }

    fn parse_op(&mut self) -> Result<Op, String> {
        let kind: OpKind = self.program[self.ip].try_into()?;
        self.ip += 1;

        if kind.has_operand() {
            return Ok(Op(kind, Some(self.extract_word()?)));
        }
        Ok(Op(kind, None))
    }

    fn extract_word(&mut self) -> Result<Word, String> {
        if self.ip + 1 > self.program_size {
            return Err(format!("could not extract word at {}", self.ip));
        }
        let word = (self.program[self.ip] as Word) << 8 | self.program[self.ip + 1] as Word;
        self.ip += size_of::<Word>();
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
        let mut machine =
            Machine::try_new(&[OpKind::Push.into(), 0x00, 0x01, OpKind::Pop.into()]).unwrap();
        machine.run(true).unwrap();
        assert!(machine.stack.pop().is_err());
    }

    #[test]
    fn test_addition() {
        let mut machine = Machine::try_new(&[
            OpKind::Push.into(),
            0x00,
            0x05,
            OpKind::Push.into(),
            0x00,
            0x03,
            OpKind::Add.into(),
        ])
        .unwrap();
        machine.run(false).unwrap();
        assert!(machine.stack.pop().is_ok_and(|value| value == 8));
    }

    #[test]
    fn test_subtrcation() {
        let mut machine = Machine::try_new(&[
            OpKind::Push.into(),
            0x00,
            0x0f,
            OpKind::Push.into(),
            0x00,
            0x0e,
            OpKind::Sub.into(),
        ])
        .unwrap();
        machine.run(false).unwrap();
        assert!(machine.stack.pop().is_ok_and(|value| value == 1));
    }

    #[test]
    fn test_multiplication() {
        let mut machine = Machine::try_new(&[
            OpKind::Push.into(),
            0x00,
            0x02,
            OpKind::Push.into(),
            0x00,
            0x0f,
            OpKind::Mul.into(),
        ])
        .unwrap();
        machine.run(false).unwrap();
        assert!(machine.stack.pop().is_ok_and(|value| value == 30));
    }

    #[test]
    fn test_division() {
        let mut machine = Machine::try_new(&[
            OpKind::Push.into(),
            0x00,
            0x0f,
            OpKind::Push.into(),
            0x00,
            0x03,
            OpKind::Div.into(),
        ])
        .unwrap();
        machine.run(false).unwrap();
        assert!(machine.stack.pop().is_ok_and(|value| value == 5));
    }

    #[test]
    fn test_division_by_zero() {
        let mut machine = Machine::try_new(&[
            OpKind::Push.into(),
            0x00,
            0x05,
            OpKind::Push.into(),
            0x00,
            0x00,
            OpKind::Div.into(),
        ])
        .unwrap();
        assert!(machine.run(false).is_err());
    }

    #[test]
    fn test_unknown_opcode() {
        let mut machine = Machine::try_new(&[0xFF]).unwrap();
        assert!(machine.run(false).is_err());
    }

    #[test]
    fn test_halt_operation() {
        let mut machine = Machine::try_new(&[OpKind::Halt.into()]).unwrap();
        machine.run(false).unwrap();
        assert!(machine.halted);
    }
}
