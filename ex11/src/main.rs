#[macro_use]
extern crate aoc_helper;

use aoc_helper::commandline::AOCApp;
use aoc_helper::file;

use std::process;

mod lib;
use lib::grid;
use lib::parser;

fn main() {
    let args = get_app_args!();
    let input_filename = args.get_input_filename();

    let input_content = file::get_content::<String>(input_filename.unwrap());
    if let Err(error) = input_content {
        println!("Got an error when reading input file: {}", error);
        process::exit(1);
    };
    let seats = input_content.unwrap();
    if seats.len() == 0 {
        println!("Input file is empty");
        process::exit(0);
    }
    let initial_grid = parser::get_seats_row(seats);
    // First part
    let mut grid = initial_grid.clone();
    grid = grid::run_while_no_diff(grid, 4, &grid::PredictionMethod::SIMPLE);
    println!(
        "The number of occupied seats using simple rules is {}",
        grid.count_items(grid::Item::OccupiedSeat)
    );
    // Second part
    let mut grid = initial_grid.clone();
    grid = grid::run_while_no_diff(grid, 5, &grid::PredictionMethod::COMPLEX);
    println!(
        "The number of occupied seats using complex rules is {}",
        grid.count_items(grid::Item::OccupiedSeat)
    );
}
