pub type Word = i16;

#[derive(Debug)]
pub enum OpType {
    Push,
    Pop,
    Echo,
    Halt,
}

impl TryFrom<u8> for OpType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Push),
            0x01 => Ok(Self::Pop),
            0x02 => Ok(Self::Echo),
            0x03 => Ok(Self::Halt),
            _ => Err("unknown OpType".to_string()),
        }
    }
}
