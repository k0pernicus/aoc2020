use std::ops;

#[derive(Copy, Clone)]
pub struct Range(pub u32, pub u32);

#[derive(Debug)]
pub struct PlanePosition(pub u32, pub u32);

impl ops::Add for Range {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Range(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
enum RowLetter {
    Front,
    Back,
    Right,
    Left,
}

impl Into<char> for RowLetter {
    fn into(self) -> char {
        self as u8 as char
    }
}

impl RowLetter {
    pub fn from(c: char) -> Result<RowLetter, ()> {
        match c {
            'F' => Ok(RowLetter::Front),
            'B' => Ok(RowLetter::Back),
            'R' => Ok(RowLetter::Right),
            'L' => Ok(RowLetter::Left),
            _ => Err(()),
        }
    }
    pub fn compute_next_step(&self, range: &Range) -> Range {
        let mid = (range.1 + range.0) / 2;
        match self {
            RowLetter::Back | RowLetter::Right => return Range(mid, range.1),
            RowLetter::Front | RowLetter::Left => return Range(range.0, mid),
        }
    }
}

pub fn get_plane_row(
    initial_row_range: &Range,
    initial_position_range: &Range,
    indication: &str,
) -> Option<PlanePosition> {
    if indication.len() != 10 {
        return None;
    }
    let indication_iter = indication.chars();
    let row = indication_iter
        .clone()
        .take(7)
        .fold(*initial_row_range, |row, c| {
            RowLetter::from(c).unwrap().compute_next_step(&row)
        });
    let position = indication_iter
        .skip(7)
        .fold(*initial_position_range, |position, c| {
            RowLetter::from(c).unwrap().compute_next_step(&position)
        });
    Some(PlanePosition(row.0, position.1))
}

pub fn get_seat_id(plane_position: &PlanePosition, row_length: u32) -> u32 {
    plane_position.0 * row_length + plane_position.1
}

/// get_missing_seat_id assumes that seat_ids is already sorted!
pub fn get_missing_seat_id(seat_ids: &Vec<u32>) -> Option<u32> {
    let nb_seats = seat_ids.len();
    for (index, seat_id) in seat_ids.iter().enumerate() {
        if index >= (nb_seats - 1) {
            break;
        }
        if seat_ids[index + 1] == seat_id + 2 {
            return Some(seat_id + 1);
        }
    }
    None
}
