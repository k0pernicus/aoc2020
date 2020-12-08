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
    if let Err(instruction_traces) = virtual_machine.run() {
        println!(
            "Found a problem - instruction traces: {:?}",
            instruction_traces
        );
        if let None = virtual_machine.debug_instruction(&instruction_traces) {
            println!("Cannot debug the sequence - bug somewhere :/");
        } else {
            println!("Correct sequence found!");
        }
    }
    virtual_machine.clean();
}
