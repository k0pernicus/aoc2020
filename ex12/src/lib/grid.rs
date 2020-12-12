use std::default;

use super::{Direction, Instruction};

#[derive(Clone, Debug)]
pub struct Point {
    x: isize,
    y: isize,
    current_direction: Direction,
}

impl default::Default for Point {
    fn default() -> Point {
        Point {
            x: 0,
            y: 0,
            current_direction: Direction::East,
        }
    }
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        let mut point = Point::default();
        point.x = x;
        point.y = y;
        point
    }
    pub fn rotate_around(&mut self, direction: &Direction, angle: i32) {
        self.current_direction = self
            .current_direction
            .compute_next_direction(direction.clone(), angle);
        match (angle, direction) {
            (0, _) | (360, _) => (),
            (180, _) => {
                self.x = -self.x;
                self.y = -self.y;
            }
            (90, Direction::Left) | (270, Direction::Right) => {
                let tmp_x = self.x;
                self.x = -self.y;
                self.y = tmp_x;
            }
            (90, Direction::Right) | (270, Direction::Left) => {
                let tmp_x = self.x;
                self.x = self.y;
                self.y = -tmp_x;
            }
            _ => println!(
                "Ooops... cannot rotate something with angle {}... bug somewhere?",
                angle
            ),
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    ship_point: Point,
    waypoint: Option<Point>,
}

impl default::Default for Grid {
    fn default() -> Grid {
        Grid {
            ship_point: Point::default(),
            waypoint: None,
        }
    }
}

impl Grid {
    pub fn get_ship_point(&self) -> Point {
        self.ship_point.clone()
    }
    pub fn new(waypoint: Option<Point>) -> Grid {
        if waypoint.is_none() {
            return Grid::default();
        }
        let mut grid = Grid::default();
        grid.waypoint = waypoint;
        grid
    }
    fn compute_next_step_without_waypoint(&mut self, instruction: &Instruction) {
        let mut tmp_direction = self.ship_point.current_direction.clone();
        match instruction.0 {
            Direction::Left | Direction::Right => {
                self.ship_point.current_direction =
                    tmp_direction.compute_next_direction(instruction.0.clone(), instruction.1);
                return;
            }
            Direction::North => tmp_direction = Direction::North,
            Direction::East => tmp_direction = Direction::East,
            Direction::West => tmp_direction = Direction::West,
            Direction::South => tmp_direction = Direction::South,
            _ => (),
        }
        match tmp_direction {
            Direction::North => self.ship_point.y -= instruction.1 as isize,
            Direction::South => self.ship_point.y += instruction.1 as isize,
            Direction::East => self.ship_point.x += instruction.1 as isize,
            Direction::West => self.ship_point.x -= instruction.1 as isize,
            _ => (),
        }
    }
    fn compute_next_step_using_waypoint(&mut self, instruction: &Instruction) {
        let mut waypoint = self.waypoint.clone().unwrap();
        let mut tmp_direction = waypoint.current_direction.clone();
        match &instruction.0 {
            d @ Direction::Left | d @ Direction::Right => {
                waypoint.rotate_around(d, instruction.1);
                self.waypoint = Some(waypoint);
                return;
            }
            Direction::North => tmp_direction = Direction::North,
            Direction::East => tmp_direction = Direction::East,
            Direction::West => tmp_direction = Direction::West,
            Direction::South => tmp_direction = Direction::South,
            Direction::Forward => {
                self.ship_point.x += instruction.1 as isize * waypoint.x;
                self.ship_point.y += instruction.1 as isize * waypoint.y;
                return;
            }
        }
        match tmp_direction {
            Direction::North => waypoint.y += instruction.1 as isize,
            Direction::South => waypoint.y -= instruction.1 as isize,
            Direction::East => waypoint.x += instruction.1 as isize,
            Direction::West => waypoint.x -= instruction.1 as isize,
            _ => (),
        }
        self.waypoint = Some(waypoint);
    }
    pub fn compute_next_step(&mut self, instruction: &Instruction) {
        if self.waypoint.is_none() {
            self.compute_next_step_without_waypoint(instruction);
            return;
        }
        self.compute_next_step_using_waypoint(instruction);
    }
    pub fn compute_manhattan_distance(&self) -> isize {
        self.ship_point.x.abs() + self.ship_point.y.abs()
    }
}
