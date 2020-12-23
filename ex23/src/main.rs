extern crate aoc_helper;

use aoc_helper::commandline::AOCApp;
use aoc_helper::file;

use std::process;

#[macro_use]
mod lib;
use lib::parser;

const NB_MOVES_DEFAULT: &'static str = "100";

fn main() {
    let mut app = AOCApp::default();
    app = app.add_argument(
        "nb_moves",
        "nb_moves",
        "n",
        false,
        true,
        "The number of moves",
    );
    let args = app.build();
    let input_filename = args.get_input_filename();
    let nb_moves = args
        .get_value_of("nb_moves")
        .unwrap_or(NB_MOVES_DEFAULT)
        .parse::<usize>()
        .unwrap();

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
    let s = raw_input.into_iter().nth(0).unwrap();
    // Part 1
    let initial_cups_part_1 = parser::get_cups(s.clone(), None);
    if initial_cups_part_1.is_err() {
        println!(
            "Got error at part 1: {:?}",
            initial_cups_part_1.unwrap_err()
        );
        process::exit(1);
    };
    let mut cups = initial_cups_part_1.unwrap();
    for i in 0..nb_moves {
        let _ = cups.play_round(i);
    }
    match cups.collect_from(1) {
        Some(ordered_cups) => println!(
            "The ordered cups for part 1 are {:?}",
            pretty_print_cups!(ordered_cups)
        ),
        None => println!("Oops... it seems the cup has not been found"),
    }
    // Part 2
    let initial_cups_part_2 = parser::get_cups(s, Some(1_000_000));
    if initial_cups_part_2.is_err() {
        println!(
            "Got error at part 2: {:?}",
            initial_cups_part_2.unwrap_err()
        );
        process::exit(1);
    }
    let mut cups = initial_cups_part_2.unwrap();
    for i in 0..10000000 {
        let _ = cups.play_round(i);
    }
    match cups.collect_from(1) {
        Some(ordered_cups) => println!(
            "The two cups next to 1 are {} and {}",
            ordered_cups.get(0).unwrap(),
            ordered_cups.get(1).unwrap()
        ),
        None => println!("Oops... it seems the cup has not been found"),
    }
}
