pub mod compute;
pub mod parser;

use std::collections::HashSet;

// 4th dimension grid
type Grid = HashSet<(isize, isize, isize, isize)>;
