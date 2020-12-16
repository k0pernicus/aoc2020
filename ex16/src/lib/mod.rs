pub mod compute;
pub mod parser;

use regex::Regex;
use std::default;
use std::str;

#[derive(Debug)]
pub struct Puzzle {
    header: Vec<ClassRanges>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl default::Default for Puzzle {
    fn default() -> Self {
        Puzzle {
            header: Vec::new(),
            my_ticket: Ticket(Vec::new()),
            nearby_tickets: Vec::new(),
        }
    }
}

impl Puzzle {
    pub fn discard_nearby_tickets(&mut self, tickets_index_to_discard: Vec<usize>) {
        tickets_index_to_discard
            .iter()
            .enumerate()
            .for_each(|(nb_tickets_removed, index)| {
                let _ = self.nearby_tickets.remove(*index - nb_tickets_removed);
            })
    }

    pub fn get_fields_on_my_ticket(&self, fields_index: &Vec<usize>) -> Vec<isize> {
        self.my_ticket
            .0
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut acc, (index, elt)| {
                if fields_index.contains(&index) {
                    acc.push(*elt);
                    return acc;
                }
                acc
            })
    }
}

#[derive(Debug)]
pub struct Ticket(Vec<isize>);

impl str::FromStr for Ticket {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(",").map(|n| n.parse::<isize>()).collect() {
            Ok(v) => {
                return Ok(Ticket(v));
            }
            Err(err) => {
                println!("Cannot parse the ticket, due to error {}", err);
                return Err(());
            }
        }
    }
}

#[derive(Debug)]
pub struct Range((isize, isize));

impl str::FromStr for Range {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let range_nb: Vec<isize> = s.split("-").map(|n| n.parse::<isize>().unwrap()).collect();
        if range_nb.len() != 2 {
            println!(
                "Expected two fields, got {} fields in string '{}'",
                range_nb.len(),
                s
            );
            return Err(());
        }
        Ok(Range((range_nb[0], range_nb[1])))
    }
}

impl Range {
    pub fn contains(&self, nb: isize) -> bool {
        let range = self.0;
        nb >= range.0 && nb <= range.1
    }
}

#[derive(Debug)]
pub struct ClassRanges {
    name: String,
    ranges: [Range; 2],
}

impl str::FromStr for ClassRanges {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RANGES_RE: Regex = Regex::new(r"(\d+-\d+) or (\d+-\d+)").unwrap();
        }
        let s_split = s.split(":").collect::<Vec<&str>>();
        if s_split.len() != 2 {
            println!("Cannot parse the string as ClassRanges, expected two initial fields, got {} in '{}'", s_split.len(), s);
            return Err(());
        }
        let name = String::from(s_split[0].trim());
        let raw_ranges = s_split[1].trim();
        let raw_ranges_matches = RANGES_RE.captures(raw_ranges);
        if let None = raw_ranges_matches {
            println!(
                "Cannot capture ranges inside second raw field in string '{}' - regex issue?",
                raw_ranges
            );
            return Err(());
        }
        let ranges_matches = raw_ranges_matches.unwrap();
        let fst_range = Range::from_str(ranges_matches.get(1).map_or("", |m| m.as_str())).unwrap();
        let snd_range = Range::from_str(ranges_matches.get(2).map_or("", |m| m.as_str())).unwrap();
        Ok(ClassRanges {
            name: name,
            ranges: [fst_range, snd_range],
        })
    }
}
