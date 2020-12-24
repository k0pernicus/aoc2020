#[macro_use]
extern crate aoc_helper;

use aoc_helper::commandline::AOCApp;
use aoc_helper::file;

use std::process;

#[macro_use]
mod lib;
use lib::TileSide;
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
    let directions = parser::parse_entries(raw_input);
    // Part 1
    let tiles = compute::Tiles::part_1(directions);
    let black_tiles = tiles.get_specific_side_tiles(TileSide::BLACK);
    println!("There are {} black tiles", black_tiles.len());
    // Part 2
    let tiles = compute::Tiles::part_2(tiles, 200, 200, 100);
    let black_tiles = tiles.get_specific_side_tiles(TileSide::BLACK);
    println!("There are {} black tiles", black_tiles.len());
}
