extern crate aoc_helper;

use aoc_helper::file;
use std::process;

mod lib;
use lib::compute;

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
    let nb_of_valid_password_part_1 = compute::compute_nb_of_valid_passwords_part_1(&v);
    println!(
        "The number of valid passwords (part 1) is {}",
        nb_of_valid_password_part_1
    );
    let nb_of_valid_password_part_2 = compute::compute_nb_of_valid_passwords_part_2(&v);
    println!(
        "The number of valid passwords (part 2) is {}",
        nb_of_valid_password_part_2
    );
}
