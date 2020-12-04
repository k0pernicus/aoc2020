extern crate aoc_helper;

use aoc_helper::file;
use std::process;

mod lib;
use lib::parser;

fn main() {
    let r = file::get_content::<String>("input");
    if let Err(error) = r {
        println!("Got an error when reading input file: {}", error);
        process::exit(1);
    };
    let v = r.unwrap();
    if v.len() == 0 {
        println!("Input file is empty");
        process::exit(0);
    }
    // Pass v to parse, as we don't care about this structure after
    let passports = parser::parse(v);
    let nb_valid_passports = passports
        .iter()
        .filter_map(|p| {
            if p.is_valid() {
                return Some(true);
            } else {
                return None;
            }
        })
        .count();
    println!("Number of valid passports: {}", nb_valid_passports);
}
