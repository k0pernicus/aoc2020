#[macro_use]
extern crate aoc_helper;

use aoc_helper::commandline::AOCApp;
use aoc_helper::file;
use std::process;

mod lib;
use lib::parser;
use lib::virtual_machine;

fn main() {
    let args = get_app_args!();
    let input_filename = args.get_input_filename();

    let input_content = file::get_content::<String>(input_filename.unwrap());
    if let Err(error) = input_content {
        println!("Got an error when reading input file: {}", error);
        process::exit(1);
    };
    let lines = input_content.unwrap();
    if lines.len() == 0 {
        println!("Input file is empty");
        process::exit(0);
    }
    let instructions = parser::parse_raw_instructions(lines);
    let mut virtual_machine = virtual_machine::VirtualMachine::new(instructions);
    match virtual_machine.run() {
        Ok(accumulator) => {
            println!("After the run, the accumulator is set to {}", accumulator);
        }
        Err(virtual_machine::VirtualMachineError::NoneInstruction()) => {
            println!("A none instruction has been detected... possibly a parse error, or an error in the input file");
        }
        Err(virtual_machine::VirtualMachineError::DetectedLoop(instruction_traces)) => {
            println!(
                "Found a problem - instruction traces: {:?} - trying to debug the sequence...",
                instruction_traces
            );
            match virtual_machine.debug_instruction(&instruction_traces) {
                Some(_) => println!("Correct sequence found!"),
                None => println!("Cannot debug the sequence - bug somewhere :/"),
            };
        }
    };
    virtual_machine.clean();
}
