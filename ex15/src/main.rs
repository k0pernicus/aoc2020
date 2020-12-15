#[macro_use]
extern crate aoc_helper;

use aoc_helper::commandline::AOCApp;
use aoc_helper::file;

use std::process;

mod lib;
use lib::{compute, parser};

const NB_TURNS: usize = 30000000;

fn main() {
    let args = get_app_args!();
    let input_filename = args.get_input_filename();

    let input_content = file::get_content::<String>(input_filename.unwrap());
    if let Err(error) = input_content {
        println!("Got an error when reading input file: {}", error);
        process::exit(1);
    };
    let raw_input = input_content.unwrap();
    if raw_input.len() == 0 {
        println!("Input file is empty");
        process::exit(0);
    } else if raw_input.len() > 1 {
        println!("Input file must be equals to one line of inputs");
        process::exit(0);
    }
    // Part 1
    let raw_line = raw_input[0].clone();
    let init_list = parser::parse(raw_line);
    let number = compute::compute_nth_turn(init_list, NB_TURNS);
    println!("The {}th number is {}.", NB_TURNS, number);
}
