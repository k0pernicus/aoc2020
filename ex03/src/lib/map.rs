use std::collections::BTreeMap;
use std::fmt;
use std::ops;

/// Coordinates representation
#[derive(Clone, Copy, Debug)]
pub struct Coordinates(pub i32, pub i32);

/// Operator overloading for Coordinates
impl ops::Add for Coordinates {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        return Coordinates(self.0 + other.0, self.1 + other.1);
    }
}

/// What is the type of "item" in the road
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum MapItem {
    Tree,
}

impl fmt::Display for MapItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tree => write!(f, "🌲"),
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

/// SlopeDirection defines the slope for one axis.
#[derive(Debug)]
pub struct SlopeDirection(Direction, i32);

impl SlopeDirection {
    pub fn new(direction: Direction, nb_boxes: i32) -> SlopeDirection {
        SlopeDirection(direction, nb_boxes)
    }
    pub fn get_coordinates(&self) -> Coordinates {
        let (direction, nb_boxes) = (&self.0, &self.1);
        match direction {
            Direction::Up => return Coordinates(-nb_boxes, 0),
            Direction::Right => return Coordinates(0, *nb_boxes),
            Direction::Down => return Coordinates(*nb_boxes, 0),
            Direction::Left => return Coordinates(-nb_boxes, 0),
        }
    }
}

/// Slope defines the direction to take in a map.
#[derive(Debug)]
pub struct Slope(SlopeDirection, SlopeDirection);

impl Slope {
    pub fn new(fst: SlopeDirection, snd: SlopeDirection) -> Slope {
        Slope(fst, snd)
    }
    pub fn get_next_direction(&self) -> Coordinates {
        self.0.get_coordinates() + self.1.get_coordinates()
    }
}

/// A grid is a two-dimensional vector that can contain a MapItem type
type Map = Vec<Vec<Option<MapItem>>>;

pub fn build_map(m: &Vec<String>) -> Map {
    m.iter()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Some(MapItem::Tree),
                    _ => None,
                })
                .collect()
        })
        .collect()
}

/// A structure to help to represent a Map type
pub struct MapMetadata {
    height: u32,
    width: u32,
}

impl MapMetadata {
    pub fn new(height: u32, width: u32) -> MapMetadata {
        MapMetadata {
            height: height,
            width: width,
        }
    }
}

/// Function to compute the number of encountered items in the map
pub fn compute_encountered_items_in_map(
    start_point: &Coordinates,
    slope: &Slope,
    map: &Map,
    map_metadata: &MapMetadata,
    stop_threshold: Option<u32>,
) -> BTreeMap<MapItem, u32> {
    let mut steps = 0;
    let mut current_position: Coordinates = *start_point;
    let mut encountered_items: BTreeMap<MapItem, u32> = BTreeMap::new();
    loop {
        if !stop_threshold.is_none() && steps > stop_threshold.unwrap() {
            break;
        }
        if current_position.0 > 0 && current_position.0 as u32 >= map_metadata.height {
            break;
        }
        // Do not count the None
        if let Some(box_item) = &map[current_position.0 as usize][current_position.1 as usize] {
            *encountered_items.entry(box_item.clone()).or_insert(0) += 1;
        }
        current_position = current_position + slope.get_next_direction();
        current_position.1 = current_position.1 % map_metadata.width as i32;
        steps += 1;
    }
    // Return the BTreeMap
    encountered_items
}
