extern crate aoc_helper;

use aoc_helper::file;
use std::process;

mod lib;
use lib::map;

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
    let map = map::build_map(&v);
    if map.is_empty() {
        println!("Input map is empty!");
        process::exit(1);
    }
    let map_metadata = map::MapMetadata::new(map.len() as u32, map[0].len() as u32);
    let start_point = map::Coordinates(0, 0);
    let fst_slop = map::SlopeDirection::new(map::Direction::Right, 3);
    let snd_slop = map::SlopeDirection::new(map::Direction::Down, 1);
    let slop = map::Slope::new(fst_slop, snd_slop);
    let encountered_items =
        map::compute_encountered_items_in_map(&start_point, &slop, &map, &map_metadata, None);
    println!("First part solution...");
    for (k, v) in encountered_items.iter() {
        println!("* encountered item '{}' {} times", k, v);
    }

    println!("Second part solution...");
    let slops = [
        map::Slope::new(
            map::SlopeDirection::new(map::Direction::Right, 1),
            map::SlopeDirection::new(map::Direction::Down, 1),
        ),
        map::Slope::new(
            map::SlopeDirection::new(map::Direction::Right, 3),
            map::SlopeDirection::new(map::Direction::Down, 1),
        ),
        map::Slope::new(
            map::SlopeDirection::new(map::Direction::Right, 5),
            map::SlopeDirection::new(map::Direction::Down, 1),
        ),
        map::Slope::new(
            map::SlopeDirection::new(map::Direction::Right, 7),
            map::SlopeDirection::new(map::Direction::Down, 1),
        ),
        map::Slope::new(
            map::SlopeDirection::new(map::Direction::Right, 1),
            map::SlopeDirection::new(map::Direction::Down, 2),
        ),
    ];
    let mut tree_items_mul: u32 = 1;
    for slop in slops.iter() {
        let encountered_items =
            map::compute_encountered_items_in_map(&start_point, &slop, &map, &map_metadata, None);
        println!("* for slop {:?}", slop);
        for (k, v) in encountered_items.iter() {
            println!("* encountered item '{}' {} times", k, v);
        }
        tree_items_mul *= encountered_items[&map::MapItem::Tree];
    }
    println!(
        "The total number of trees (multiplied) is {}",
        tree_items_mul
    );
}
