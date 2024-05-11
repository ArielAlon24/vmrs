use std::collections::HashMap;
use std::iter::Peekable;
use std::mem::size_of;
use std::str::Chars;

use vmrs::OpKind;
use vmrs::Word;

pub struct Preprocessor<'a> {
    iterator: Peekable<Chars<'a>>,
    byte: Word,
    debug: bool,
}

impl<'a> Preprocessor<'a> {
    pub fn new(unicode: &'a str, debug: bool) -> Self {
        Self {
            iterator: unicode.chars().peekable(),
            byte: 0,
            debug,
        }
    }

    fn skip_space(&mut self) {
        while self.iterator.peek().is_some_and(|c| c.is_whitespace()) {
            self.iterator.next();
        }
    }

    fn skip_comment(&mut self) {
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

    fn skip_word(&mut self) -> Result<(), String> {
        let mut num = String::new();
        while self
            .iterator
            .peek()
            .is_some_and(|c| c.is_digit(10) || c == &'.')
        {
            num.push(self.iterator.next().unwrap());
        }
        num.parse::<Word>()
            .map_err(|_| "could not parse number".to_string())?;
        Ok(())
    }

    fn skip_op(&mut self) -> Result<(), String> {
        let kind: OpKind = self.next_identifier().to_uppercase().try_into()?;

        self.skip_space();

        if kind == OpKind::Goto || kind == OpKind::Goif {
            self.skip_space();
            self.next_identifier();
            self.byte += size_of::<Word>() as i16;
        } else if kind.has_operand() {
            self.skip_word()?;
            self.byte += size_of::<Word>() as i16;
        }
        self.byte += 1;

        self.skip_space();

        if self.debug {
            println!("[DEBUG preprocess] {} | {:?}", self.byte, kind);
        }
        Ok(())
    }

    pub fn preprocess(&mut self) -> Result<HashMap<String, Word>, String> {
        let mut labels = HashMap::new();

        while let Some(c) = self.iterator.peek() {
            match c {
                c if c.is_whitespace() => {
                    self.iterator.next().unwrap();
                }
                '|' => self.skip_comment(),
                '@' => {
                    self.iterator.next().unwrap();
                    labels.insert(self.next_identifier(), self.byte);
                }
                _ => self.skip_op()?,
            }
        }

        return Ok(labels);
    }
}
