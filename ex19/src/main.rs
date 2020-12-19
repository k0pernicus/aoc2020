#[macro_use]
extern crate aoc_helper;
extern crate rayon;
extern crate regex;

use rayon::prelude::*;

use aoc_helper::commandline::AOCApp;
use aoc_helper::file;

use std::collections::HashSet;
use std::process;

#[macro_use]
mod lib;
use lib::parser;

const NUM_CPU: usize = 2;

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
    let proposal_index = parser::find_proposal_index(&raw_input);
    let mut proposal = raw_input.clone()[proposal_index..].to_vec();
    proposal.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
    let nb_matches = proposal.len();

    // Do not eat all the CPU
    rayon::ThreadPoolBuilder::new()
        .num_threads(NUM_CPU)
        .build_global()
        .unwrap();

    // Part 1
    let rules = parser::parse_rules(raw_input[0..proposal_index].to_vec(), None);
    println!(">> Rules have been computed...");
    let nb_matches_part_1 = proposal
        .par_iter()
        .enumerate()
        .fold(
            || 0usize,
            |acc, (index, s)| {
                println!("... computing rule {} / {}", index, nb_matches);
                if rules.is_match(0, s) {
                    return acc + 1;
                }
                return acc;
            },
        )
        .sum::<usize>();
    println!("> Number of matches for part 1: {}", nb_matches_part_1);
    // Part 2
    let rules = parser::parse_rules(raw_input[0..proposal_index].to_vec(), Some(set![8, 11]));
    let re = rules.get_rule(0);
    // Free the memory
    rules.drop();
    println!(">> Rules have been computed...");
    let mut acc = 0;
    for (index, c_match) in proposal.iter().enumerate() {
        println!("Computing rule {} / {}", index, nb_matches);
        if re.is_match(c_match) {
            acc += 1;
        };
    }
    println!("> Number of matches for part 2: {}", acc);
}
