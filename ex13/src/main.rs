#[macro_use]
extern crate aoc_helper;

use aoc_helper::commandline::AOCApp;
use aoc_helper::file;

use std::process;

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
    // Part 1
    let (user_timestamp, bus_ids) = parser::parse_raw_input(&raw_input);
    let (bus_id_to_take, at_timestamp) = compute::find_earliest_bus(&user_timestamp, bus_ids);
    let min_to_wait = at_timestamp.clone() - user_timestamp;
    println!(
        "Part 1: You must take the bus {} at {} - you will have to wait {} minutes",
        bus_id_to_take, at_timestamp, min_to_wait
    );
    println!(
        "> The solution is {}",
        (min_to_wait.get_value() * bus_id_to_take.get_value())
    );
    // Part 2
    let bus_with_internal_departures = parser::parse_with_internal_departures(&raw_input);
    let first_timestamp =
        compute::find_earliest_timestamp_for_all_buses(bus_with_internal_departures);
    println!(
        "> The earliest timestamp for all buses (to solve the second part of the exercise) is {}",
        first_timestamp
    );
}
