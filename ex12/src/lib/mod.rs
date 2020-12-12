pub mod grid;
pub mod parser;

use std::convert;
use std::fmt;
use std::str;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Direction {
    pub fn compute_next_direction(&self, direction: Direction, mut rotation: i32) -> Direction {
        if direction == Direction::Left {
            rotation = rotation * -1;
        }
        let c_rotation = match self {
            Direction::North => 0,
            Direction::East => 90,
            Direction::South => 180,
            Direction::West => 270,
            e @ _ => {
                println!("Ooops... Found {} in compute_next_direction!", e);
                0i32
            }
        };
        Direction::from(rotation + c_rotation)
    }
}

impl convert::From<char> for Direction {
    fn from(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'N' => Self::North,
            'S' => Self::South,
            'E' => Self::East,
            'W' => Self::West,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => Self::Forward,
        }
    }
}

impl convert::From<i32> for Direction {
    fn from(u: i32) -> Self {
        if u == 0 || u % 360 == 0 {
            return Self::North;
        }
        if u % 270 == 0 {
            if u >= 0 {
                return Self::West;
            }
            return Self::East;
        }
        if u % 180 == 0 {
            return Self::South;
        }
        if u % 90 == 0 {
            if u >= 0 {
                return Self::East;
            }
            return Self::West;
        }
        println!("Convertion of value {} has been avoided...", u);
        return Self::North;
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::North => write!(f, "N"),
            Direction::South => write!(f, "S"),
            Direction::East => write!(f, "E"),
            Direction::West => write!(f, "W"),
            Direction::Left => write!(f, "L"),
            Direction::Right => write!(f, "R"),
            Direction::Forward => write!(f, "F"),
        }
    }
}

pub struct Instruction(Direction, i32);

impl str::FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            println!(
                "The number of characters per string must be at least 2, got {}",
                s
            );
            return Err(());
        }
        let direction = s.chars().take(1).next();
        if direction.is_none() {
            println!("No direction when parsing the string {}", s);
            return Err(());
        }
        let move_cases = s
            .chars()
            .skip_while(|c| c.is_alphabetic())
            .collect::<String>()
            .parse::<i32>();
        if move_cases.is_err() {
            println!("Error when parsing the number of moves in the string {}", s);
            return Err(());
        }
        Ok(Instruction(
            Direction::from(direction.unwrap()),
            move_cases.unwrap(),
        ))
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl Instruction {
    pub fn is_rotate(&self) -> bool {
        (self.0 == Direction::Left) || (self.0 == Direction::Right)
    }
}
