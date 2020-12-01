extern crate aoc_helper;

use std::process;
use aoc_helper::file;

mod lib;
use lib::compute;

fn main() {
	let r = file::get_content::<i32>("input");
	if let Err(error) = r {
		println!("Got an error when reading input file: {}", error);
        	process::exit(1);
	};
	let v = r.unwrap();
    	if v.len() == 0 {
        	println!("Input file is empty");
        	process::exit(0);
    	}
	if let Some((entry_1, entry_2)) = compute::get_two_entries_that_sum(&v, 2020) {
		println!("Found tuple ({}, {})", entry_1, entry_2);
		println!("The result of the multiplication is {}", entry_1 * entry_2);
	} else {
		println!("No entries found");
	}
	if let Some((entry_1, entry_2, entry_3)) = compute::get_three_entries_that_sum(&v, 2020) {
		println!("Found tuple ({}, {}, {})", entry_1, entry_2, entry_3);
		println!("The result of the multiplication is {}", entry_1 * entry_2 * entry_3);
	} else {
		println!("No entries found");
	}

}
