use super::{Directions, TileSide};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Tiles {
    map: HashMap<(isize, isize), TileSide>,
}

fn count_adjacent_black_tiles(pos: &(isize, isize), tiles: &Tiles) -> usize {
    let mut adjacent = 0;
    for candidate in [(1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)].iter() {
        if let Some(v) = tiles.map.get(&(pos.0 + candidate.0, pos.1 + candidate.1)) {
            if *v == TileSide::BLACK {
                adjacent += 1;
            }
        }
    }
    adjacent
}

impl Tiles {
    pub fn part_1(directions: Vec<Directions>) -> Tiles {
        let mut tiles = Tiles {
            map: HashMap::new(),
        };
        for direction in directions.iter() {
            let tile_coordinates = direction.follow_path();
            tiles
                .map
                .entry(tile_coordinates)
                .or_insert(TileSide::default())
                .flip();
        }
        tiles
    }
    pub fn part_2(mut tiles: Tiles, height: isize, width: isize, nb_days: usize) -> Tiles {
        // Initialize
        let mid_width = (width / 2) + 1;
        let mid_height = (height / 2) + 1;
        // Fill with white
        for x in -mid_width..mid_width {
            for y in -mid_height..mid_height {
                if let None = tiles.map.get(&(x, y)) {
                    tiles.map.insert((x, y), TileSide::default());
                }
            }
        }
        for _day in 1..=nb_days {
            let mut positions = Vec::new();
            for (position, side) in tiles.map.iter() {
                let adjacent_black_tiles = count_adjacent_black_tiles(position, &tiles);
                if (*side == TileSide::BLACK
                    && (adjacent_black_tiles == 0 || adjacent_black_tiles > 2))
                    || (*side == TileSide::WHITE && adjacent_black_tiles == 2)
                {
                    positions.push(position.clone());
                }
            }
            for position in positions.iter() {
                tiles.map.get_mut(position).unwrap().flip();
            }
        }
        tiles
    }
    pub fn get_specific_side_tiles(&self, tile_side: TileSide) -> Vec<(isize, isize)> {
        self.map
            .iter()
            .filter_map(|(coor, c_side)| {
                if c_side == &tile_side {
                    return Some(*coor);
                } else {
                    return None;
                }
            })
            .collect()
    }
}
