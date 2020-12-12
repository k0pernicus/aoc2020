#[macro_use]
extern crate aoc_helper;

use aoc_helper::commandline::AOCApp;
use aoc_helper::file;

use std::process;

mod lib;
use lib::{grid, parser};

fn main() {
    let args = get_app_args!();
    let input_filename = args.get_input_filename();

    let input_content = file::get_content::<String>(input_filename.unwrap());
    if let Err(error) = input_content {
        println!("Got an error when reading input file: {}", error);
        process::exit(1);
    };
    let raw_instructions = input_content.unwrap();
    if raw_instructions.len() == 0 {
        println!("Input file is empty");
        process::exit(0);
    }
    let instructions = match parser::parse_instructions(raw_instructions) {
        Ok(instructions) => instructions,
        Err(_) => {
            println!("Error when parsing the inputs");
            process::exit(1);
        }
    };
    let mut grid = grid::Grid::default();
    for instruction in instructions.iter() {
        // if instruction.is_rotation() {
        //     println!("BEFORE > {:?}", grid);
        // }
        grid.compute_next_step(instruction);
        // if instruction.is_rotation() {
        //     println!("Instruction: {}", instruction);
        //     println!("AFTER > {:?}", grid);
        // }
    }
    println!(
        "The manhattan distance is {}",
        grid.compute_manhattan_distance()
    );
}
