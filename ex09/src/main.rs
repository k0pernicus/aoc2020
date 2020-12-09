extern crate aoc_helper;

use aoc_helper::commandline::AOCApp;
use aoc_helper::file;
use std::process;

mod lib;
use lib::compute;

const DEFAULT_NB_OF_PREAMBLES: &'static str = "25";

fn main() {
    let mut app: AOCApp = AOCApp::default();
    app = app.add_argument(
        "nb-of-preambles",
        "nb-of-preambles",
        "n",
        false,
        true,
        "set the number of preambles for the input file",
    );
    let args = app.build();
    let input_filename = args.get_input_filename();
    let nb_of_preambles = args
        .get_value_of("nb-of-preambles")
        .unwrap_or(DEFAULT_NB_OF_PREAMBLES)
        .parse::<usize>()
        .unwrap();

    let input_content = file::get_content::<u32>(input_filename.unwrap());
    if let Err(error) = input_content {
        println!("Got an error when reading input file: {}", error);
        process::exit(1);
    };
    let input = input_content.unwrap();
    if input.len() == 0 {
        println!("Input file is empty");
        process::exit(0);
    }
    let first_non_sum_nb = compute::get_first_non_sum_nb(&input, nb_of_preambles);
    if let None = first_non_sum_nb {
        println!("Error: Cannot find a non-sum number... Mistake in input?");
        process::exit(1);
    } else {
        let (non_sum_nb, non_sum_nb_index) = first_non_sum_nb.unwrap();
        println!(
            "The non-sum number is {} (index at {} of the input list)",
            non_sum_nb, non_sum_nb_index
        );
        let invalid_range =
            compute::get_invalid_range_memoization(&input[..non_sum_nb_index], non_sum_nb).unwrap();
        let data_range = &input[invalid_range.0..invalid_range.1];
        let min_value = data_range.iter().min().unwrap();
        let max_value = data_range.iter().max().unwrap();
        println!("The sum of the range is {}", min_value + max_value);
    }
}
