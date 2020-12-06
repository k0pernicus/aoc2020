extern crate aoc_helper;

use std::process;

mod lib;
use lib::{compute, parser};

use aoc_helper::{commandline, file};

fn main() {
    let app = commandline::AOCApp::new("ex06", "0.1.0", "k0pernicus");
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
    let answers = parser::parse_groups(lines);

    // Part I
    let sum_yes_questions: u32 = answers
        .iter()
        .map(|vector_of_answers| vector_of_answers.concat())
        .map(|answer| compute::count_questions_with_yes_response(answer.as_str()))
        .sum();
    println!("The sum of the 'yes' questions is {}", sum_yes_questions);

    // Part II
    let sum_duplicate_responses_per_group: u32 = answers
        .iter()
        .map(|vector_of_answers| (vector_of_answers.concat(), vector_of_answers.len() as u32))
        .map(|(answer, nb_persons)| {
            compute::count_questions_with_duplicate_yes_reponse(answer.as_str(), nb_persons)
        })
        .sum();
    println!(
        "The sum of duplicates 'yes' questions per group is {}",
        sum_duplicate_responses_per_group
    );
}
