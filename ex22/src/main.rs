#[macro_use]
extern crate aoc_helper;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use aoc_helper::commandline::AOCApp;
use aoc_helper::file;

use std::process;

#[macro_use]
mod lib;
use lib::{compute, parser};

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
    let deques = parser::get_deques(raw_input);
    if deques.is_err() {
        println!("Error when parsing the inputs");
        process::exit(1);
    }
    let deques = deques.unwrap();
    // Part 1
    let deques_part_1 = deques.clone();
    match compute::get_winner_part_1(deques_part_1.0, deques_part_1.1) {
        Ok((winner, score)) => {
            println!("Step 1: Player {} won with a score of {}", winner, score)
        }
        Err(_) => {
            println!("Exiting...");
            return;
        }
    };
    // Part 2
    match compute::get_winner_part_2(deques.0, deques.1) {
        Ok((winner, score)) => {
            println!("Step 2: Player {} won with a score of {}", winner, score)
        }
        Err(_) => {
            println!("Exiting...");
            return;
        }
    }
}
