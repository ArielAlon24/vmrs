use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::iter::Peekable;
use std::mem::size_of;
use std::process::exit;
use std::str::Chars;
use vmrs::{Op, OpKind, Word};
type Bytes = Vec<u8>;

struct Assembler<'a> {
    iterator: Peekable<Chars<'a>>,
    symbol_table: HashMap<String, Word>,
    byte: Word,
    row: usize,
    col: usize,
}

impl<'a> Assembler<'a> {
    pub fn new(unicode: &'a str) -> Self {
        Self {
            iterator: unicode.chars().peekable(),
            symbol_table: HashMap::new(),
            byte: 0,
            row: 1,
            col: 0,
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
        let identifier = self.next_identifier();

        if self.symbol_table.contains_key(&identifier) {
            return Err(format!(
                "tried to add the label: '{}' for the second time",
                identifier
            ));
        }

        self.symbol_table.insert(identifier, self.byte);
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
            match self.symbol_table.get(&label) {
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
        println!(
            "[DEBUG] (byte: {:0>3} | row: {:0>3}, col: {:0>3}) - (row: {:0>3}, col: {:0>3}) | {:?}",
            self.byte, srow, scol, self.row, self.col, op
        );

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

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args[1].as_str()).expect("could not open src");
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
                .expect("could not open out");
            file.write(&bytes).expect("could not write to out");
        }
    }
}
