pub mod compute;
pub mod parser;

use std::default;
use std::str;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TileSide {
    BLACK,
    WHITE,
}

impl TileSide {
    pub fn flip(&mut self) {
        match self {
            TileSide::BLACK => *self = TileSide::WHITE,
            TileSide::WHITE => *self = TileSide::BLACK,
        }
    }
}

impl default::Default for TileSide {
    fn default() -> Self {
        TileSide::WHITE
    }
}

pub enum ErrorDirection {
    NoMatch,
}

#[derive(Debug)]
pub enum Direction {
    EAST,
    SOUTHEAST,
    SOUTHWEST,
    WEST,
    NORTEAST,
    NORTHWEST,
}

impl str::FromStr for Direction {
    type Err = ErrorDirection;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "e" => Ok(Direction::EAST),
            "se" => Ok(Direction::SOUTHEAST),
            "sw" => Ok(Direction::SOUTHWEST),
            "w" => Ok(Direction::WEST),
            "ne" => Ok(Direction::NORTEAST),
            "nw" => Ok(Direction::NORTHWEST),
            _ => {
                println!("Error: '{}' is not recognized as a direction", s);
                return Err(ErrorDirection::NoMatch);
            }
        }
    }
}

impl Direction {
    pub fn to_coord(&self) -> (isize, isize) {
        match self {
            Direction::EAST => (1, 0),
            Direction::WEST => (-1, 0),
            Direction::NORTEAST => (1, -1),
            Direction::NORTHWEST => (0, -1),
            Direction::SOUTHEAST => (0, 1),
            Direction::SOUTHWEST => (-1, 1),
        }
    }
}

#[derive(Debug)]
pub struct Directions(Vec<Direction>);

impl Directions {
    pub fn follow_path(&self) -> (isize, isize) {
        self.0.iter().fold((0isize, 0isize), |acc, direction| {
            let follow_coord = direction.to_coord();
            (acc.0 + follow_coord.0, acc.1 + follow_coord.1)
        })
    }
}

impl default::Default for Directions {
    fn default() -> Self {
        Directions(Vec::new())
    }
}

impl str::FromStr for Directions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut acc: String = String::with_capacity(2);
        let mut directions = Directions::default();
        for c in s.chars() {
            acc.push(c);
            if c == 'e' || c == 'w' {
                match Direction::from_str(acc.as_str()) {
                    Ok(direction) => directions.0.push(direction),
                    Err(_) => return Err(()),
                }
                acc.clear();
            }
        }
        Ok(directions)
    }
}
