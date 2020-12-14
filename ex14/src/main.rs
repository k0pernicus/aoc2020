#[macro_use]
extern crate aoc_helper;
extern crate bit_array;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate typenum;

use aoc_helper::commandline::AOCApp;
use aoc_helper::file;

use std::collections::HashMap;
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
    let program_instructions = parser::parse_program(raw_input).unwrap();

    // First part
    let mut memory_addresses = HashMap::<usize, isize>::new();
    for (mask, instructions) in program_instructions.iter() {
        for instruction in instructions.iter() {
            let bit_array = compute::get_bit_array(instruction.value);
            println!("> Instruction: {:?}", bit_array);
            let bit_array_with_mask = mask.first_part_decode(bit_array);
            println!(">> After applying the mask: {:?}", bit_array_with_mask);
            let bit_array_with_mask_digit = compute::to_digit(&bit_array_with_mask);
            if !memory_addresses.contains_key(&instruction.memory_address) {
                memory_addresses.insert(instruction.memory_address, bit_array_with_mask_digit);
            } else {
                *memory_addresses
                    .get_mut(&instruction.memory_address)
                    .unwrap() = bit_array_with_mask_digit;
            }
        }
    }
    println!(
        "The sum of all non-zero memory addresses is {}",
        memory_addresses.values().sum::<isize>()
    );

    println!("#################");

    // Second part
    // let mut sum = 0;
    memory_addresses.clear();
    for (mask, instructions) in program_instructions {
        for instruction in instructions.iter() {
            let bit_array = compute::get_bit_array(instruction.value);
            println!("> Instruction: {:?}", bit_array);
            let memory_address_as_bit_array = compute::get_bit_array(instruction.memory_address);
            let shuffled_memory_addresses = mask
                .second_part_decode(memory_address_as_bit_array)
                .iter()
                .map(|address| compute::to_digit(&address) as usize)
                .collect::<Vec<usize>>();
            println!(">> Memory addresses: {:?}", shuffled_memory_addresses);
            for shuffled_memory_address in shuffled_memory_addresses.iter() {
                if !memory_addresses.contains_key(shuffled_memory_address) {
                    memory_addresses.insert(*shuffled_memory_address, instruction.value as isize);
                } else {
                    *memory_addresses.get_mut(&shuffled_memory_address).unwrap() =
                        instruction.value as isize;
                }
            }
        }
        // sum += memory_addresses.values().sum::<isize>();
        // memory_addresses.clear();
    }
    println!(
        "The sum of all non-zero memory addresses is {}",
        memory_addresses.values().sum::<isize>()
    );
}
