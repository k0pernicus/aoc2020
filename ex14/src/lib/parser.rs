use super::{Instruction, Instructions, Mask};

use std::str::FromStr;

trait IsSpecificInstruction {
    fn is_mask(&self) -> bool;
    fn is_memory_set(&self) -> bool;
}

impl IsSpecificInstruction for String {
    fn is_mask(&self) -> bool {
        self.starts_with("mask")
    }
    fn is_memory_set(&self) -> bool {
        self.starts_with("mem")
    }
}

pub fn parse_program(program_instructions: Vec<String>) -> Result<Vec<(Mask, Instructions)>, ()> {
    if program_instructions.len() < 2 {
        println!("Not enough informations to process");
        return Err(());
    }
    let mut instructions = Vec::new();
    let mut c_mask: Mask = Mask::default();
    let mut c_instructions: Vec<Instruction> = Vec::new();
    for instruction in program_instructions.iter() {
        if instruction.is_mask() {
            instructions.push((c_mask, c_instructions.clone()));
            c_instructions.clear();
            c_mask = Mask::from_str(instruction.as_str()).unwrap();
        } else if instruction.is_memory_set() {
            c_instructions.push(Instruction::from_str(instruction.as_str()).unwrap());
        } else {
            println!(
                "Program instruction '{}' is incorrect... skipping",
                instruction
            );
        }
    }
    instructions.push((c_mask, c_instructions.clone()));
    Ok(instructions)
}
