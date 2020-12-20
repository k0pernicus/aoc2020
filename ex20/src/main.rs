#[macro_use]
extern crate aoc_helper;
#[macro_use]
extern crate lazy_static;

use aoc_helper::commandline::AOCApp;
use aoc_helper::file;

use std::collections::{HashMap, HashSet};
use std::process;

#[macro_use]
mod lib;
use lib::{image, parser};

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
    let images = parser::parse_inputs(raw_input).unwrap();
    let images_ids = images.keys();
    let mut positions = lib::Positions::new();
    // Compute the combinations
    for (i, id) in images_ids.enumerate() {
        let c_image = images.get(id).unwrap();
        let c_image_combinations = c_image.get_combinations();
        for (c_image_combination_index, c_image_combination) in
            c_image_combinations.iter().enumerate()
        {
            let mut is_match = false;
            for (other_image_id, other_image) in images.iter() {
                if id == other_image_id {
                    continue;
                }
                let combinations = other_image.get_combinations();
                for (combination_index, combination) in combinations.iter().enumerate() {
                    // println!("Tried to match...");
                    // println!("{}", c_image_combination);
                    // println!("{}", combination);

                    if c_image_combination
                        .match_with_existing_corners(image::CORNER::TOP, &combination)
                    {
                        positions.add_position(
                            *id,
                            *other_image_id,
                            c_image_combination_index,
                            combination_index,
                            image::CORNER::TOP,
                        );
                        is_match = true;
                        break;
                    }
                }
                for (combination_index, combination) in combinations.iter().enumerate() {
                    if c_image_combination
                        .match_with_existing_corners(image::CORNER::BOTTOM, &combination)
                    {
                        positions.add_position(
                            *id,
                            *other_image_id,
                            c_image_combination_index,
                            combination_index,
                            image::CORNER::BOTTOM,
                        );
                        is_match = true;
                        break;
                    }
                }
                for (combination_index, combination) in combinations.iter().enumerate() {
                    if c_image_combination
                        .match_with_existing_corners(image::CORNER::LEFT, &combination)
                    {
                        positions.add_position(
                            *id,
                            *other_image_id,
                            c_image_combination_index,
                            combination_index,
                            image::CORNER::LEFT,
                        );
                        is_match = true;
                        break;
                    }
                }
                for (combination_index, combination) in combinations.iter().enumerate() {
                    if c_image_combination
                        .match_with_existing_corners(image::CORNER::RIGHT, &combination)
                    {
                        positions.add_position(
                            *id,
                            *other_image_id,
                            c_image_combination_index,
                            combination_index,
                            image::CORNER::RIGHT,
                        );
                        is_match = true;
                        break;
                    }
                }
            }
        }
    }
    let ok_positions = positions.draw();
    let mut nb_positions_per_case: HashMap<usize, HashSet<usize>> = HashMap::new();
    for ok_hashset in ok_positions.iter() {
        for (id, positions) in ok_hashset.iter() {
            nb_positions_per_case
                .entry(positions.len())
                .or_insert(HashSet::new())
                .insert(*id);
        }
    }
    let corners = nb_positions_per_case.get(&2).unwrap();
    let mul = corners.iter().fold(1usize, |acc, value| acc * value);
    println!("{:?} -> {}", corners, mul);
}
