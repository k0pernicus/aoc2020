use std::convert;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Item {
    OccupiedSeat,
    EmptySeat,
    Floor,
}

impl convert::From<char> for Item {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::EmptySeat,
            '#' => Self::OccupiedSeat,
            _ => Self::Floor,
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OccupiedSeat => write!(f, "#"),
            Self::EmptySeat => write!(f, "L"),
            Self::Floor => write!(f, "."),
        }
    }
}

pub enum PredictionMethod {
    SIMPLE,
    COMPLEX,
}

impl Item {
    pub fn predict_next_position(
        &self,
        coordinates: (usize, usize),
        nb_seats_to_check: u32,
        grid: &Grid,
        method: &PredictionMethod,
    ) -> Self {
        match method {
            PredictionMethod::SIMPLE => {
                self.predict(coordinates, nb_seats_to_check, grid, get_nb_adjacent_seats)
            }
            PredictionMethod::COMPLEX => self.predict(
                coordinates,
                nb_seats_to_check,
                grid,
                get_nb_first_adjacent_seats,
            ),
        }
    }

    fn predict(
        &self,
        coordinates: (usize, usize),
        nb_seats_to_check: u32,
        grid: &Grid,
        prediction_fn: fn(&Grid, (usize, usize), Item) -> u32,
    ) -> Self {
        let nb_occupied_seats = prediction_fn(grid, coordinates, Item::OccupiedSeat);
        if *self == Self::EmptySeat && nb_occupied_seats == 0 {
            return Self::OccupiedSeat;
        } else if *self == Self::OccupiedSeat && nb_occupied_seats >= nb_seats_to_check {
            return Item::EmptySeat;
        }
        self.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid {
    rows: Vec<Row>,
    height: usize,
    width: usize,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows.iter() {
            for c in row.iter() {
                let _ = write!(f, "{}", c);
            }
            let _ = write!(f, "\n");
        }
        return Ok(());
    }
}

impl Grid {
    pub fn new(v: Vec<Row>) -> Grid {
        let height = v.len();
        let mut width = 0;
        if height > 0 {
            width = v[0].len();
        }
        Grid {
            rows: v,
            height,
            width,
        }
    }

    pub fn count_items(&self, item: Item) -> u32 {
        self.rows
            .iter()
            .map(|row| {
                row.iter().fold(0u32, |mut sum, c_item| {
                    if *c_item == item {
                        sum += 1;
                        return sum;
                    }
                    sum
                })
            })
            .sum()
    }
}

pub type Row = Vec<Item>;

pub fn run_round(g: Grid, nb_seats_to_check: u32, prediction_method: &PredictionMethod) -> Grid {
    let mut new_rows = g.rows.clone();
    for (row_index, row) in g.rows.iter().enumerate() {
        for (seat_index, seat) in row.iter().enumerate() {
            new_rows[row_index][seat_index] = seat.predict_next_position(
                (row_index, seat_index),
                nb_seats_to_check,
                &g,
                prediction_method,
            );
        }
    }
    Grid::new(new_rows)
}

pub fn run_while_no_diff(
    mut g: Grid,
    nb_seats_to_check: u32,
    prediction_method: &PredictionMethod,
) -> Grid {
    let mut previous_g = g.clone();
    g = run_round(g, nb_seats_to_check, prediction_method);
    while g != previous_g {
        previous_g = g.clone();
        g = run_round(g, nb_seats_to_check, prediction_method);
    }
    g
}

fn get_nb_adjacent_seats(grid: &Grid, coordinates: (usize, usize), seat: Item) -> u32 {
    let i_coordinates: (isize, isize) = (coordinates.0 as isize, coordinates.1 as isize);
    // Generate all the coordinates
    vec![
        (i_coordinates.0 - 1, i_coordinates.1 - 1),
        (i_coordinates.0 - 1, i_coordinates.1),
        (i_coordinates.0 - 1, i_coordinates.1 + 1),
        (i_coordinates.0, i_coordinates.1 - 1),
        (i_coordinates.0, i_coordinates.1 + 1),
        (i_coordinates.0 + 1, i_coordinates.1),
        (i_coordinates.0 + 1, i_coordinates.1 + 1),
        (i_coordinates.0 + 1, i_coordinates.1 - 1),
    ]
    .iter()
    .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < grid.height as isize && *y < grid.width as isize)
    .filter(|c_coordinates| grid.rows[c_coordinates.0 as usize][c_coordinates.1 as usize] == seat)
    .count() as u32
}

pub fn get_nb_first_adjacent_seats(grid: &Grid, coordinates: (usize, usize), seat: Item) -> u32 {
    let mut sum = 0;
    // Generate all the coordinates + checks
    for y in -1..2 {
        for x in -1..2 {
            // Do not check the current item...
            if x == y && x == 0 {
                continue;
            }
            let (mut c_y, mut c_x) = (coordinates.0 as isize + y, coordinates.1 as isize + x);
            // Check that the coordinates are valid
            while c_x >= 0 && c_y >= 0 && c_x < grid.width as isize && c_y < grid.height as isize {
                let c_item = grid.rows[c_y as usize][c_x as usize].clone();
                if c_item != Item::Floor {
                    if c_item == seat {
                        sum += 1;
                    }
                    break;
                }
                c_y += y;
                c_x += x;
            }
        }
    }
    sum
}
