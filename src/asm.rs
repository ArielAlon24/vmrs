use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::iter::Peekable;
use std::process::exit;
use std::str::Chars;
use vmrs::{Op, OpKind, Word};

type Bytes = Vec<u8>;

struct Assembler<'a> {
    iterator: Peekable<Chars<'a>>,
    row: usize,
    col: usize,
}

impl<'a> Assembler<'a> {
    pub fn new(unicode: &'a str) -> Self {
        Self {
            iterator: unicode.chars().peekable(),
            row: 1,
            col: 0,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.iterator.peek() {
            match c {
                c if c == &'\n' => self.row += 1,
                c if c.is_whitespace() => {}
                _ => break,
            }
            self.iterator.next();
        }

        while self
            .iterator
            .peek()
            .is_some_and(|c| c.is_whitespace() && c != &'\n')
        {
            self.iterator.next();
        }
    }

    fn next_comment(&mut self) {
        while self.iterator.peek().is_some_and(|c| c != &'\n') {
            self.iterator.next();
        }
    }

    fn next_identifier(&mut self) -> String {
        let mut name = String::new();
        while self.iterator.peek().is_some_and(|c| c.is_alphabetic()) {
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
            num.push(self.iterator.next().unwrap());
        }

        Ok(num
            .parse()
            .map_err(|_| "could not parse number".to_string())?)
    }

    fn assemble_op(&mut self) -> Result<Op, String> {
        let kind: OpKind = self.next_identifier().to_uppercase().try_into()?;

        self.skip_whitespace();

        let op = match kind.has_operand() {
            true => Op(kind, Some(self.next_word()?)),
            false => Op(kind, None),
        };
        println!("[DEBUG]: ({}, {}) {:?}", self.row, self.col, op);
        self.skip_whitespace();
        Ok(op)
    }

    pub fn assemble(&mut self) -> Result<Bytes, String> {
        let mut bytes = Vec::new();

        while let Some(c) = self.iterator.peek() {
            match c {
                c if c.is_whitespace() => self.skip_whitespace(),
                c if c == &'|' => self.next_comment(),
                _ => {
                    let op = self.assemble_op()?;
                    bytes.append(&mut op.into());
                }
            }
        }

        return Ok(bytes);
    }
}

fn main() {
    let mut file = File::open("test.asm").expect("could not open file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("empty file supplied");
    let unicode = String::from_utf8(buffer).expect("could not read unicode contents");

    let mut assembler = Assembler::new(&unicode);
    let result = assembler.assemble();
    match result {
        Err(message) => {
            eprintln!("ERROR: {}", message);
            exit(1);
        }
        Ok(bytes) => {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open("test.o")
                .expect("could not open out file");
            file.write(&bytes).expect("could not write to out");
        }
    }
}
