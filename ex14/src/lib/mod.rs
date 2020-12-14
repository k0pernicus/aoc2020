pub mod compute;
pub mod parser;

use lazy_static;
use regex::Regex;
use std::str;

pub struct Instruction {
    memory_address: usize,
    value: usize,
}

impl str::FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref INSTRUCTION_RE: Regex = Regex::new(r"^mem[(\d+)] = (\d+)$").unwrap();
        }
        match INSTRUCTION_RE.captures(s) {
            Some(captures) => Ok(Instruction {
                memory_address: captures
                    .get(1)
                    .map_or("", |m| m.as_str())
                    .parse::<usize>()
                    .unwrap(),
                value: captures
                    .get(2)
                    .map_or("", |m| m.as_str())
                    .parse::<usize>()
                    .unwrap(),
            }),
            None => {
                println!("Cannot parse instruction {} using regex", s);
                Err(())
            }
        }
    }
}

type Instructions = Vec<Instruction>;
