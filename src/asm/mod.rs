pub mod assembler;
pub mod preprocessor;

use assembler::{Assembler, Bytes};
use preprocessor::Preprocessor;

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::process::exit;

const DEBUG: bool = false;

fn run(unicode: &str) -> Result<Bytes, String> {
    let mut preprocessor = Preprocessor::new(&unicode, DEBUG);
    let lables = preprocessor.preprocess()?;

    let mut assembler = Assembler::new(&unicode, lables, DEBUG);
    assembler.assemble()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args[1].as_str()).expect("could not open src");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("empty file supplied");
    let unicode = String::from_utf8(buffer).expect("could not read unicode contents");

    match run(&unicode) {
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
