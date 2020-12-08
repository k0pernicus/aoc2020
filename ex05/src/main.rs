extern crate aoc_helper;

use aoc_helper::{commandline, file};
use compute::PlanePosition;
use std::process;

mod lib;
use lib::compute;

fn main() {
    // Helper to initialize a CLI
    let main_app = commandline::AOCApp::new("ex05", "0.1.0", "k0pernicus");
    let args = main_app.build();
    let filename = args.get_input_filename();

    let r = file::get_content::<String>(filename.unwrap());
    if let Err(error) = r {
        println!("Got an error when reading input file: {}", error);
        process::exit(1);
    };
    let v = r.unwrap();
    if v.len() == 0 {
        println!("Input file is empty");
        process::exit(0);
    }
    let initial_row_range = compute::Range(0, 128);
    let initial_position_range = compute::Range(0, 7);

    // Part I
    let mut seat_ids = v
        .iter()
        .map(|indication| {
            let plane_position = compute::get_plane_row(
                &initial_row_range,
                &initial_position_range,
                indication.as_str(),
            )
            .unwrap_or(PlanePosition(0, 0));
            return compute::get_seat_id(&plane_position, 8u32);
        })
        .collect::<Vec<u32>>();
    let max_seat_it = seat_ids.iter().max();
    if let Some(seat_id) = max_seat_it {
        println!("The max seat ID is {}", seat_id);
    } else {
        println!("There is no max seat ID... bug somewhere :/");
    }

    // Part II
    seat_ids.sort();
    if let Some(missing_seat_id) = compute::get_missing_seat_id(&seat_ids) {
        println!("The missing seat ID is {}", missing_seat_id);
    } else {
        println!("There is no missing seat ID... bug somewhere :/");
    }
}
