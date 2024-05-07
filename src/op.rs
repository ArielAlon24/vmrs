pub type Word = i16;

pub struct OpCode {}
impl OpCode {
    pub const PUSH: u8 = 0x00;
    pub const POP: u8 = 0x01;
    pub const ECHO: u8 = 0x02;
    pub const ADD: u8 = 0x03;
    pub const SUB: u8 = 0x04;
    pub const MUL: u8 = 0x05;
    pub const DIV: u8 = 0x06;
    pub const HALT: u8 = 0x07;
}

#[derive(Debug)]
pub enum Op {
    Push(Word),
    Pop,
    Echo,
    Add,
    Sub,
    Mul,
    Div,
    Halt,
}
