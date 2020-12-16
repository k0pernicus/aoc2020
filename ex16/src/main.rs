#[macro_use]
extern crate aoc_helper;
extern crate regex;
#[macro_use]
extern crate lazy_static;

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
    let mut puzzle = parser::parse_input(raw_input);
    // Part 1
    let wrong_values = compute::get_wrong_values_in_nearby_tickets(&puzzle);
    println!("Found following wrong values: {:?}", wrong_values);
    println!(
        "The sum of those is {}",
        wrong_values.iter().map(|(_, value)| value).sum::<isize>()
    );
    // Part 2
    let tickets_index_to_discard = wrong_values
        .into_iter()
        .map(|(index, _)| index)
        .collect::<Vec<usize>>();
    puzzle.discard_nearby_tickets(tickets_index_to_discard);
    let header_order = compute::get_header_order(&puzzle);
    println!("header order: {:?}", header_order);
    let departure_header_ordered: Vec<usize> = header_order
        .into_iter()
        .enumerate()
        .map(|(index, x)| (index, x.starts_with("departure")))
        .filter(|(_, starts_with_predicate)| *starts_with_predicate)
        .map(|(index, _)| index)
        .collect();
    println!("departures header order: {:?}", departure_header_ordered);
    let fields = puzzle.get_fields_on_my_ticket(&departure_header_ordered);
    println!(
        "The sum of the fields that begin by 'departure' is {}",
        fields.iter().fold(1isize, |acc, value| acc * value)
    );
}
