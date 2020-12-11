use super::grid::{Grid, Item, Row};

pub fn get_seats_row(row_lines: Vec<String>) -> Grid {
    Grid::new(
        row_lines
            .iter()
            .map(|line| line.chars().map(|c| Item::from(c)).collect::<Vec<Item>>())
            .collect::<Vec<Row>>(),
    )
}
