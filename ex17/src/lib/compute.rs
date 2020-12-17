use super::Grid;
use std::collections::HashMap;

fn compute_step(g: &Grid, nb_dimensions: usize) -> Grid {
    let mut hashmap = HashMap::new();
    // Compute the number of neighbors per active state
    for (x, y, z, w) in g.iter() {
        for dx in if nb_dimensions > 0 { -1..=1 } else { 0..=0 } {
            for dy in if nb_dimensions > 1 { -1..=1 } else { 0..=0 } {
                for dz in if nb_dimensions > 2 { -1..=1 } else { 0..=0 } {
                    for dw in if nb_dimensions > 3 { -1..=1 } else { 0..=0 } {
                        if ![dx, dy, dz, dw].iter().all(|d| *d == 0) {
                            let p = (x + dx, y + dy, z + dz, w + dw);
                            hashmap.insert(p, hashmap.get(&p).unwrap_or(&0) + 1);
                        }
                    }
                }
            }
        }
    }
    // Now, filter and update the HashSet in order to return the new active states (2 or 3 neighbors)
    hashmap
        .iter()
        .filter_map(|(coordinates, n_neighbors)| {
            if *n_neighbors == 3 || (*n_neighbors == 2 && g.contains(coordinates)) {
                // Those are considered as active
                Some(*coordinates)
            } else {
                // Unactive
                None
            }
        })
        .collect()
}

pub fn get_left_cubes(g: &Grid, nb_steps: usize, nb_dimensions: usize) -> Grid {
    let mut tmp_grid = g.clone();
    for _ in 0..nb_steps {
        tmp_grid = compute_step(&tmp_grid, nb_dimensions);
    }
    tmp_grid
}
