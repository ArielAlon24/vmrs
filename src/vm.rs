pub mod machine;
pub mod op;
pub mod stack;

use machine::Machine;

fn main() {
    let program: [u8; 5] = [0, 0, 12, 2, 3];

    let mut machine = Machine::try_new(&program).expect("oops");

    if let Err(error) = machine.execute() {
        eprintln!("ERROR: {}", error);
    }
}
