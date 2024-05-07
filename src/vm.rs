use std::env;
use std::fs;
use std::process::exit;
use vmrs::Machine;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        exit(1);
    }

    let path = &args[1];
    let result = fs::read(path);

    if result.is_err() {
        eprintln!("ERROR: could not read file");
        exit(1);
    }

    let mut machine = Machine::try_new(&result.unwrap()).expect("oops");

    if let Err(error) = machine.run(true) {
        eprintln!("ERROR: {}", error);
        exit(1);
    }
}
