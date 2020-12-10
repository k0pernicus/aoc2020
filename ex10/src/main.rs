#[macro_use]
extern crate aoc_helper;

use aoc_helper::commandline::AOCApp;
use aoc_helper::file;
use std::process;

mod lib;
use lib::compute;

fn main() {
    let args = get_app_args!();
    let input_filename = args.get_input_filename();

    let input_content = file::get_content::<usize>(input_filename.unwrap());
    if let Err(error) = input_content {
        println!("Got an error when reading input file: {}", error);
        process::exit(1);
    };
    let mut adapters = input_content.unwrap();
    if adapters.len() == 0 {
        println!("Input file is empty");
        process::exit(0);
    }
    adapters.sort();
    let adapter_differences = compute::get_jolts_difference_per_adapter(&adapters);
    println!("adapter_differences: {:?}", adapter_differences);
    if let Some(solution_part_1) = compute::get_solution_to_part_1(adapter_differences) {
        println!("The solution to part 1 is {}", solution_part_1);
    } else {
        println!(
            "Oops... It seems something wrong happened when computing the adapters differences :/"
        );
        process::exit(1);
    }
    let nb_paths = compute::get_nb_of_simple_paths_backtracking(adapters);
    println!("The number of paths is {}", nb_paths);
}
