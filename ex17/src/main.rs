#[macro_use]
extern crate aoc_helper;

use aoc_helper::commandline::AOCApp;
use aoc_helper::file;

use std::process;

mod lib;
use lib::{compute, parser};

const NB_STEPS: usize = 6;

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
    }
    let initial_grid = parser::parse_lines(raw_input);
    let left_cubes_part_one = compute::get_left_cubes(&initial_grid, NB_STEPS, 3);
    println!(
        "Part 1: the number of left cubes is {}",
        left_cubes_part_one.len()
    );
    let left_cubes_part_two = compute::get_left_cubes(&initial_grid, NB_STEPS, 4);
    println!(
        "Part 2: the number of left cubes is {}",
        left_cubes_part_two.len()
    );
}
