use std::collections::HashMap;
use std::iter::Peekable;
use std::mem::size_of;
use std::str::Chars;
use vmrs::{Op, OpKind, Word};

pub type Bytes = Vec<u8>;

pub struct Assembler<'a> {
    iterator: Peekable<Chars<'a>>,
    labels: HashMap<String, Word>,
    byte: Word,
    row: usize,
    col: usize,
    debug: bool,
}

impl<'a> Assembler<'a> {
    pub fn new(unicode: &'a str, labels: HashMap<String, Word>, debug: bool) -> Self {
        Self {
            iterator: unicode.chars().peekable(),
            labels,
            byte: 0,
            row: 1,
            col: 0,
            debug,
        }
    }

    fn new_line(&mut self) {
        self.row += 1;
        self.col = 0;
        self.iterator.next();
    }

    fn skip_space(&mut self) {
        while self.iterator.peek().is_some_and(is_space) {
            self.iterator.next();
            self.col += 1;
        }
    }

    fn next_comment(&mut self) {
        while self.iterator.peek().is_some_and(|c| c != &'\n') {
            self.col += 1;
            self.iterator.next();
        }
    }

    fn next_identifier(&mut self) -> String {
        let mut name = String::new();
        while self.iterator.peek().is_some_and(|c| c.is_alphabetic()) {
            self.col += 1;
            name.push(self.iterator.next().unwrap());
        }
        name
    }

    fn next_word(&mut self) -> Result<Word, String> {
        let mut num = String::new();
        while self
            .iterator
            .peek()
            .is_some_and(|c| c.is_digit(10) || c == &'.')
        {
            self.col += 1;
            num.push(self.iterator.next().unwrap());
        }

        Ok(num
            .parse()
            .map_err(|_| "could not parse number".to_string())?)
    }

    fn next_label(&mut self) -> Result<(), String> {
        self.iterator.next().unwrap(); // going over '@'
        self.next_identifier();
        self.byte += size_of::<Word>() as i16;
        Ok(())
    }

    fn assemble_op(&mut self) -> Result<Op, String> {
        let (srow, scol) = (self.row, self.col);
        let kind: OpKind = self.next_identifier().to_uppercase().try_into()?;

        self.skip_space();

        let mut operand = None;

        if kind == OpKind::Goto || kind == OpKind::Goif {
            self.skip_space();
            let label = self.next_identifier();
            match self.labels.get(&label) {
                None => return Err(format!("unrecognized label: '{}'", label)),
                Some(&address) => operand = Some(address),
            }
            self.byte += size_of::<Word>() as i16;
        } else if kind.has_operand() {
            operand = Some(self.next_word()?);
            self.byte += size_of::<Word>() as i16;
        }
        self.byte += 1;

        let op = Op(kind, operand);

        self.skip_space();
        if self.debug {
            println!(
            "[DEBUG] (byte: {:0>3} | row: {:0>3}, col: {:0>3}) - (row: {:0>3}, col: {:0>3}) | {:?}",
            self.byte, srow, scol, self.row, self.col, op
        );
        }

        Ok(op)
    }

    pub fn assemble(&mut self) -> Result<Bytes, String> {
        let mut bytes = Vec::new();

        while let Some(c) = self.iterator.peek() {
            match c {
                c if is_space(c) => self.skip_space(),
                '|' => self.next_comment(),
                '\n' => self.new_line(),
                '@' => self.next_label()?,
                _ => bytes.append(&mut self.assemble_op()?.into()),
            }
        }

        return Ok(bytes);
    }
}

fn is_space(c: &char) -> bool {
    c == &' ' || c == &'\t'
}
