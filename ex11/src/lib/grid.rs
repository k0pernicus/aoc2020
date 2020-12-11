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
                self.predict_next_position_simple(coordinates, nb_seats_to_check, grid)
            }
            PredictionMethod::COMPLEX => {
                self.predict_next_position_complex(coordinates, nb_seats_to_check, grid)
            }
        }
    }

    fn predict_next_position_simple(
        &self,
        coordinates: (usize, usize),
        nb_seats_to_check: u32,
        grid: &Grid,
    ) -> Self {
        if *self == Self::EmptySeat
            && grid.get_nb_adjacent_seats(coordinates, Item::OccupiedSeat) == 0
        {
            return Self::OccupiedSeat;
        } else if *self == Self::OccupiedSeat
            && grid.get_nb_adjacent_seats(coordinates, Item::OccupiedSeat) >= nb_seats_to_check
        {
            return Item::EmptySeat;
        }
        self.clone()
    }

    fn predict_next_position_complex(
        &self,
        coordinates: (usize, usize),
        nb_seats_to_check: u32,
        grid: &Grid,
    ) -> Self {
        if *self == Self::EmptySeat
            && grid.get_first_adjacent_seats(coordinates, Item::OccupiedSeat) == 0
        {
            return Self::OccupiedSeat;
        } else if *self == Self::OccupiedSeat
            && grid.get_first_adjacent_seats(coordinates, Item::OccupiedSeat) >= nb_seats_to_check
        {
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
    pub fn get_nb_adjacent_seats(&self, coordinates: (usize, usize), seat: Item) -> u32 {
        let i_coordinates: (isize, isize) = (coordinates.0 as isize, coordinates.1 as isize);
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
        .filter(|(x, y)| {
            *x >= 0 && *y >= 0 && *x < self.height as isize && *y < self.width as isize
        })
        .filter(|c_coordinates| {
            self.rows[c_coordinates.0 as usize][c_coordinates.1 as usize] == seat
        })
        .count() as u32
    }

    pub fn get_first_adjacent_seats(&self, coordinates: (usize, usize), seat: Item) -> u32 {
        let mut seats = Vec::new();

        for y in -1..2 {
            for x in -1..2 {
                if x == y && x == 0 {
                    continue;
                }
                let (mut c_y, mut c_x) = (coordinates.0 as isize + y, coordinates.1 as isize + x);
                while c_x >= 0
                    && c_y >= 0
                    && c_x < self.width as isize
                    && c_y < self.height as isize
                {
                    let c_item = self.rows[c_y as usize][c_x as usize].clone();
                    if c_item != Item::Floor {
                        seats.push(c_item);
                        break;
                    }
                    c_y += y;
                    c_x += x;
                }
            }
        }

        seats.iter().fold(0u32, |mut sum, c_seat| {
            if *c_seat == seat {
                sum += 1;
                return sum;
            } else {
                return sum;
            }
        })
    }

    pub fn count_seats(&self, seat: Item) -> u32 {
        self.rows
            .iter()
            .map(|row| {
                row.iter().fold(0u32, |mut sum, c_seat| {
                    if *c_seat == seat {
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
