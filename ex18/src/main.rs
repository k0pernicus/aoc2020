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
    }
    // Part 1
    let sum_expr = raw_input.iter().fold(0isize, |acc, expression| {
        acc + expr::evaluate_expression(expression, true)
    });
    println!("The sum of all expressions for part 1 is {}", sum_expr);
    // Part 2
    let sum_expr = raw_input.iter().fold(0isize, |acc, expression| {
        acc + expr::evaluate_expression(expression, false)
    });
    println!("The sum of all expressions for part 2 is {}", sum_expr);
}
