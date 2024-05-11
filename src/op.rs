pub type Word = i16;

#[derive(Debug, PartialEq, Eq)]
pub enum OpKind {
    /* Basic Stack Operations */
    Push,
    Pop,
    Echo,

    /* Arithematic */
    Add,
    Sub,
    Mul,
    Div,

    /* Navigation */
    Goto,
    Goif,

    /* Other */
    Copy,
    Halt,
}

impl TryFrom<u8> for OpKind {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(OpKind::Push),
            0x01 => Ok(OpKind::Pop),
            0x02 => Ok(OpKind::Echo),
            0x03 => Ok(OpKind::Add),
            0x04 => Ok(OpKind::Sub),
            0x05 => Ok(OpKind::Mul),
            0x06 => Ok(OpKind::Div),
            0x07 => Ok(OpKind::Goto),
            0x08 => Ok(OpKind::Goif),
            0x09 => Ok(OpKind::Copy),
            0x0a => Ok(OpKind::Halt),
            _ => Err(format!("unknown binary op kind: '{}'", value)),
        }
    }
}

impl TryFrom<String> for OpKind {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "PUSH" => Ok(OpKind::Push),
            "POP" => Ok(OpKind::Pop),
            "ECHO" => Ok(OpKind::Echo),
            "ADD" => Ok(OpKind::Add),
            "SUB" => Ok(OpKind::Sub),
            "MUL" => Ok(OpKind::Mul),
            "DIV" => Ok(OpKind::Div),
            "GOTO" => Ok(OpKind::Goto),
            "GOIF" => Ok(OpKind::Goif),
            "COPY" => Ok(OpKind::Copy),
            "HALT" => Ok(OpKind::Halt),

            _ => Err(format!("unknown string op kind: '{}'", value)),
        }
    }
}

impl Into<u8> for OpKind {
    fn into(self) -> u8 {
        match self {
            OpKind::Push => 0x00,
            OpKind::Pop => 0x01,
            OpKind::Echo => 0x02,
            OpKind::Add => 0x03,
            OpKind::Sub => 0x04,
            OpKind::Mul => 0x05,
            OpKind::Div => 0x06,
            OpKind::Goto => 0x07,
            OpKind::Goif => 0x08,
            OpKind::Copy => 0x09,
            OpKind::Halt => 0x0a,
        }
    }
}

impl OpKind {
    pub fn has_operand(&self) -> bool {
        match self {
            OpKind::Push => true,
            OpKind::Pop => false,
            OpKind::Echo => false,
            OpKind::Add => false,
            OpKind::Sub => false,
            OpKind::Mul => false,
            OpKind::Div => false,
            OpKind::Goto => true,
            OpKind::Goif => true,
            OpKind::Copy => false,
            OpKind::Halt => false,
        }
    }
}

#[derive(Debug)]
pub struct Op(pub OpKind, pub Option<Word>);

impl Into<Vec<u8>> for Op {
    fn into(self) -> Vec<u8> {
        let mut vec: Vec<u8> = vec![self.0.into()];
        if let Some(word) = self.1 {
            vec.append(&mut word.to_be_bytes().to_vec());
        }
        vec
    }
}
