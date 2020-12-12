use std::default;

use super::{Direction, Instruction};

#[derive(Debug)]
pub struct Grid {
    x: isize,
    y: isize,
    current_direction: Direction,
}

impl default::Default for Grid {
    fn default() -> Grid {
        Grid {
            x: 0,
            y: 0,
            current_direction: Direction::East,
        }
    }
}

impl Grid {
    fn add_next_step(&mut self, direction: Direction, move_cases: isize) {
        match direction {
            Direction::North => self.y -= move_cases,
            Direction::South => self.y += move_cases,
            Direction::East => self.x += move_cases,
            Direction::West => self.x -= move_cases,
            _ => (),
        }
    }
    pub fn compute_next_step(&mut self, instruction: &Instruction) {
        let mut tmp_direction = self.current_direction.clone();
        match instruction.0 {
            Direction::Left | Direction::Right => {
                self.current_direction =
                    tmp_direction.compute_next_direction(instruction.0.clone(), instruction.1);
                return;
            }
            Direction::North => tmp_direction = Direction::North,
            Direction::East => tmp_direction = Direction::East,
            Direction::West => tmp_direction = Direction::West,
            Direction::South => tmp_direction = Direction::South,
            _ => (),
        }
        self.add_next_step(tmp_direction, instruction.1 as isize);
    }
    pub fn compute_manhattan_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}
