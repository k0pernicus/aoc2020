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
    if raw_input.len() < 2 || raw_input.len() > 2 {
        println!("Input file is empty");
        process::exit(0);
    }
    let (door_public_key, card_public_key) =
        parser::get_public_keys(raw_input.get(0).unwrap(), raw_input.get(1).unwrap());
    if door_public_key.is_err() || card_public_key.is_err() {
        println!("Error when parsing the input(s)");
        process::exit(1);
    }
    let (door_public_key, card_public_key) = (door_public_key.unwrap(), card_public_key.unwrap());
    let door_loop_size = match compute::compute_loop_size(7, door_public_key) {
        Some(loop_size) => loop_size,
        None => {
            println!("Cannot compute the loop size from the door public key");
            process::exit(1);
        }
    };
    let card_loop_size = match compute::compute_loop_size(7, card_public_key) {
        Some(loop_size) => loop_size,
        None => {
            println!("Cannot compute the loop size from the card public key");
            process::exit(1);
        }
    };
    let door_encryption_key = compute::compute_encryption_key(door_public_key, card_loop_size);
    let card_encryption_key = compute::compute_encryption_key(card_public_key, door_loop_size);
    if door_encryption_key != card_encryption_key {
        println!("Oops... it seems there was an error when computing the loops :/");
        process::exit(1);
    }
    println!("The encryption key is {}", door_encryption_key);
}
