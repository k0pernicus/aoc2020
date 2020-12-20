pub mod image;
pub mod parser;

use std::collections::{HashMap, HashSet, VecDeque};

pub struct Positions {
    hash: HashMap<(usize, usize), HashMap<(usize, usize), image::CORNER>>,
}

impl Positions {
    pub fn new() -> Self {
        Positions {
            hash: HashMap::new(),
        }
    }
    pub fn add_position(
        &mut self,
        a: usize,
        b: usize,
        a_dim: usize,
        b_dim: usize,
        direction: image::CORNER,
    ) {
        self.hash
            .entry((a, a_dim))
            .or_insert(HashMap::new())
            .entry((b, b_dim))
            .or_insert(direction);
    }
    pub fn draw(&self) -> Vec<HashMap<usize, HashSet<image::CORNER>>> {
        let mut ok = Vec::new();
        for ((a, a_dim), other_cards) in self.hash.iter() {
            let mut to_visit: VecDeque<(usize, usize)> = VecDeque::new();
            let mut visited: VecDeque<(usize, usize)> = VecDeque::new();
            let mut directions: HashMap<usize, HashSet<image::CORNER>> = HashMap::new();
            let mut is_not_correct = false;
            // println!("Computing for {} ({})...", a, a_dim);
            visited.push_back((*a, *a_dim));
            for ((b, b_dim), direction) in other_cards.iter() {
                directions
                    .entry(*a)
                    .or_insert(HashSet::new())
                    .insert(direction.clone());
                to_visit.push_back((*b, *b_dim));
            }
            while !to_visit.is_empty() {
                let item = to_visit.pop_back().unwrap();
                visited.push_back(item);
                if !self.hash.contains_key(&item) {
                    is_not_correct = true;
                    break;
                }
                let neighbors = self.hash.get(&item).unwrap();
                for (neighbor, direction) in neighbors.iter() {
                    directions
                        .entry(item.0)
                        .or_insert(HashSet::new())
                        .insert(direction.clone());
                    if !visited.contains(neighbor) {
                        to_visit.push_back(*neighbor);
                    }
                }
            }
            if is_not_correct {
                continue;
            }
            ok.push(directions);
        }
        ok
    }
}
