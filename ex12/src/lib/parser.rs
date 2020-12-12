use super::Instruction;
use std::str::FromStr;

pub fn parse_instructions(instructions: Vec<String>) -> Result<Vec<Instruction>, ()> {
    instructions
        .iter()
        .map(|instruction| Instruction::from_str(instruction.as_str()))
        .collect()
}
