pub mod compute;
pub mod parser;

use bit_array::BitArray;
use lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::default;
use std::str;
use typenum::U36;

#[derive(Clone, Debug)]
pub struct Instruction {
    pub memory_address: usize,
    value: usize,
}

impl str::FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref INSTRUCTION_RE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
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

#[derive(Debug)]
pub struct Mask {
    overwrite: HashMap<usize, u8>,
    length: usize,
}

impl str::FromStr for Mask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref MASK_RE: Regex = Regex::new(r"^mask = ([[:alnum:]]+)$").unwrap();
        }
        let captures = MASK_RE.captures(s);
        if captures.is_none() {
            println!("No capture found in {}...", s);
            return Err(());
        }
        let raw_mask = captures.unwrap().get(1).map_or("", |m| m.as_str());
        let mut mask = Mask::default();
        mask.length = raw_mask.len();
        raw_mask
            .chars()
            .rev()
            .enumerate()
            .for_each(|(index, c)| match c {
                '0' => {
                    let _ = mask.overwrite.insert(index, 0u8);
                }
                '1' => {
                    let _ = mask.overwrite.insert(index, 1u8);
                }
                _ => (),
            });
        Ok(mask)
    }
}

impl default::Default for Mask {
    fn default() -> Self {
        Mask {
            length: 0,
            overwrite: HashMap::new(),
        }
    }
}

impl Mask {
    pub fn apply_to(&self, mut bit_array: BitArray<u32, U36>) -> BitArray<u32, U36> {
        for (index, bit) in self.overwrite.iter() {
            let chain_index = bit_array.len() - (*index + 1);
            match *bit {
                1 => {
                    bit_array.set(chain_index, true);
                }
                _ => {
                    bit_array.set(chain_index, false);
                }
            }
        }
        bit_array
    }
}

type Instructions = Vec<Instruction>;
