use super::Grid;

// Returns a HashSet of x, y, z and w (for the 4th dimension)
pub fn parse_lines(v: Vec<String>) -> Grid {
    v.iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars().enumerate().filter_map(move |(y, c)| {
                if c == '#' {
                    Some((x as isize, y as isize, 0, 0))
                } else {
                    None
                }
            })
        })
        .collect()
}
