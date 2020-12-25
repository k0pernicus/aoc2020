use std::num::ParseIntError;

pub fn get_public_keys(
    door_public_key: &String,
    card_public_key: &String,
) -> (Result<usize, ParseIntError>, Result<usize, ParseIntError>) {
    (door_public_key.parse(), card_public_key.parse())
}
