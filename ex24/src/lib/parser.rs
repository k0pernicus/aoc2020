use super::Directions;
use std::str::FromStr;

pub fn parse_entries(v: Vec<String>) -> Vec<Directions> {
    let mut directions = Vec::new();
    for s in v.iter() {
        if s.is_empty() {
            continue;
        }
        match Directions::from_str(s) {
            Ok(c_directions) => directions.push(c_directions),
            Err(_) => {
                println!("Error when parsing input...");
                return directions;
            }
        }
    }
    directions
}
