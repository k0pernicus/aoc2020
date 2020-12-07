extern crate aoc_helper;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use aoc_helper::{commandline, file};
use std::process;

mod lib;
use lib::parser;

const BAG_NAME: &'static str = "shiny gold";

fn main() {
    let app = commandline::AOCApp::new("ex07", "0.1.0", "k0pernicus");
    let args = app.build();
    let input_filename = args.get_input_filename();

    let input_content = file::get_content::<String>(input_filename.unwrap());
    if let Err(error) = input_content {
        println!("Got an error when reading input file: {}", error);
        process::exit(1);
    };
    let lines = input_content.unwrap();
    if lines.len() == 0 {
        println!("Input file is empty");
        process::exit(0);
    }
    let bag_rules = parser::parse_file(lines).unwrap();
    let parents_of_shiny_gold = bag_rules.find_parent_bags(BAG_NAME);
    println!(
        "Number of parents for shiny gold: {}",
        parents_of_shiny_gold.len()
    );
    let total_nb_bags = bag_rules.compute_sum_contained_nb_bags(BAG_NAME) - 1;
    println!("The total number of bags to buy is {}", total_nb_bags);
}
