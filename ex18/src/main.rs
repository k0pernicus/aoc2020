#[macro_use]
extern crate aoc_helper;

use aoc_helper::commandline::AOCApp;
use aoc_helper::file;

use std::process;

mod lib;
use lib::expr;

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
    println!(
        "Result of the expression: {}",
        expr::evaluate_expression(raw_input[0].as_str())
    );
}
