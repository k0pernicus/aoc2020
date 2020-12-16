use super::{ClassRanges, Puzzle, Ticket};
use std::str::FromStr;

trait GetLineType {
    fn is_header(self) -> bool;
    fn defines_my_ticket(self) -> bool;
    fn defines_nearby_ticket(self) -> bool;
}

impl GetLineType for &str {
    fn is_header(self) -> bool {
        return self.contains(":") && self.contains("or");
    }
    fn defines_my_ticket(self) -> bool {
        return self.contains("your ticket");
    }
    fn defines_nearby_ticket(self) -> bool {
        return self.contains("nearby ticket");
    }
}

fn parse_header(s: &str) -> ClassRanges {
    ClassRanges::from_str(s).unwrap()
}

fn parse_ticket(s: &str) -> Ticket {
    Ticket::from_str(s).unwrap()
}

pub fn parse_input(input: Vec<String>) -> Puzzle {
    let mut puzzle = Puzzle::default();
    let mut is_my_ticket: bool = false;
    let mut is_nearby_ticket: bool = false;
    for input_line in input {
        match (is_my_ticket, is_nearby_ticket) {
            (true, _) => {
                puzzle.my_ticket = parse_ticket(input_line.as_str());
                is_my_ticket = false;
            }
            (_, true) => {
                puzzle
                    .nearby_tickets
                    .push(parse_ticket(input_line.as_str()));
            }
            _ => {
                if input_line.is_empty() {
                    continue;
                }
                if input_line.is_header() {
                    puzzle.header.push(parse_header(input_line.as_str()));
                    continue;
                }
                if input_line.defines_my_ticket() {
                    is_my_ticket = true;
                    continue;
                }
                if input_line.defines_nearby_ticket() {
                    is_nearby_ticket = true;
                    continue;
                }
            }
        }
    }
    puzzle
}
