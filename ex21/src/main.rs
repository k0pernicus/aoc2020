#[macro_use]
extern crate aoc_helper;

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
    let foods = parser::parse_input(raw_input).unwrap();
    let (ingredients_without_allergenes, ingredients_per_allergenes) =
        compute::solve_part_1_and_2(&foods);
    println!(
        "The {} ingredients without allergenes are {:?}: total of **{:?}** ingredients for all food",
        ingredients_without_allergenes.len(),
        ingredients_without_allergenes,
        ingredients_without_allergenes.values().sum::<usize>()
    );
    // By default, a BTreeMap is sorted by keys
    let sorted_ingredients = ingredients_per_allergenes
        .iter()
        .map(|(_, ingredient)| ingredient.as_str())
        .collect::<Vec<&str>>();
    println!(
        "The canonical dangerous ingredient list is **{}**",
        sorted_ingredients.join(",")
    );
}
