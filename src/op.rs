pub type Word = i16;

pub struct OpCode {}
impl OpCode {
    pub const PUSH: u8 = 0x00;
    pub const POP: u8 = 0x01;
    pub const ECHO: u8 = 0x02;
    pub const HALT: u8 = 0x03;
}

#[derive(Debug)]
pub enum Op {
    Push(Word),
    Pop,
    Echo,
    Halt,
}
