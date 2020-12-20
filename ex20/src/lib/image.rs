use regex::Regex;
use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum CORNER {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}

#[derive(Clone, Debug)]
pub struct Image {
    pub id: usize,
    length: usize,
    filled_case: HashSet<(usize, usize)>,
}

impl Image {
    pub fn from_raw_image(v: &Vec<String>) -> Result<Image, ()> {
        lazy_static! {
            static ref TILE_ID_RE: Regex = Regex::new(r"Tile (\d+):").unwrap();
        }
        if v.len() == 0 {
            println!("Cannot convert an empty vector to an Image struct");
            return Err(());
        }
        let tile_id = match TILE_ID_RE.captures(&v[0]) {
            Some(captures) => captures.get(1).map_or("", |m| m.as_str()),
            None => {
                println!("No capture for tile id");
                return Err(());
            }
        };
        let mut image = Image {
            id: tile_id.parse::<usize>().unwrap(),
            length: v.len() - 1,
            filled_case: HashSet::new(),
        };
        for y in 0..v.len() {
            let v_y = v.len() - y - 1;
            if v_y == 0 {
                break;
            }
            let filled_case_x: Vec<usize> = v[v_y]
                .chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '#' => Some(x),
                    _ => None,
                })
                .collect();
            for x in filled_case_x {
                image.filled_case.insert((y, x));
            }
        }
        Ok(image)
    }

    pub fn get_corners(&self) -> HashSet<(usize, usize)> {
        let mut cases = self.filled_case.clone();
        cases.retain(|(y, x)| {
            (*y == (self.length - 1) || *y == 0) || (*x == (self.length - 1) || *x == 0)
        });
        cases
    }

    pub fn rotate_right(&self) -> Image {
        Image {
            id: self.id,
            length: self.length,
            filled_case: self
                .filled_case
                .iter()
                .map(|(y, x)| (*x, self.length - 1 - *y))
                .collect(),
        }
    }

    pub fn rotate_left(&self) -> Image {
        Image {
            id: self.id,
            length: self.length,
            filled_case: self
                .filled_case
                .iter()
                .map(|(y, x)| (self.length - 1 - *x, *y))
                .collect(),
        }
    }

    pub fn flip_horizontal(&self) -> Image {
        Image {
            id: self.id,
            length: self.length,
            filled_case: self
                .filled_case
                .iter()
                .map(|(y, x)| (*y, self.length - 1 - *x))
                .collect(),
        }
    }

    pub fn flip_vertical(&self) -> Image {
        Image {
            id: self.id,
            length: self.length,
            filled_case: self
                .filled_case
                .iter()
                .map(|(y, x)| (self.length - 1 - *y, *x))
                .collect(),
        }
    }

    pub fn get_corners_with_position(&self, position: &CORNER) -> HashSet<usize> {
        self.filled_case
            .iter()
            .fold(HashSet::new(), |mut acc, (y, x)| {
                match position {
                    CORNER::BOTTOM => {
                        if *y == 0 {
                            acc.insert(*x);
                        }
                    }
                    CORNER::TOP => {
                        if *y == self.length - 1 {
                            acc.insert(*x);
                        }
                    }
                    CORNER::LEFT => {
                        if *x == 0 {
                            acc.insert(*y);
                        }
                    }
                    CORNER::RIGHT => {
                        if *x == self.length - 1 {
                            acc.insert(*y);
                        }
                    }
                }
                return acc;
            })
    }

    pub fn match_with_existing_corners(&self, corner: CORNER, other_image: &Image) -> bool {
        let self_corners = self.get_corners_with_position(&corner);
        let other_corners = match corner {
            CORNER::TOP => other_image.get_corners_with_position(&CORNER::BOTTOM),
            CORNER::BOTTOM => other_image.get_corners_with_position(&CORNER::TOP),
            CORNER::LEFT => other_image.get_corners_with_position(&CORNER::RIGHT),
            CORNER::RIGHT => other_image.get_corners_with_position(&CORNER::LEFT),
        };
        // TODO: Filter on x / y
        // println!("CORNERS:");
        // println!("{:?} vs {:?}", other_corners, self_corners);
        other_corners == self_corners
    }

    pub fn get_combinations(&self) -> Vec<Image> {
        let mut combinations: Vec<Image> = Vec::new();
        let mut c_image = self.clone();
        for i in 0..5 {
            combinations.push(c_image.clone());
            combinations.push(c_image.flip_horizontal());
            combinations.push(c_image.flip_vertical());
            c_image = c_image.rotate_left();
        }
        combinations
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "Tile {}:\n", self.id);
        for y in 0..self.length {
            for x in 0..self.length {
                if self.filled_case.contains(&(self.length - 1 - y, x)) {
                    let _ = write!(f, "#");
                } else {
                    let _ = write!(f, ".");
                }
            }
            let _ = write!(f, "\n");
        }
        Ok(())
    }
}
