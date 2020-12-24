use super::vm::Instruction;
use std::str::FromStr;

pub fn parse_raw_instructions(v: Vec<String>) -> Vec<Instruction> {
    v.iter()
        .map(|raw_instruction| Instruction::from_str(raw_instruction.as_str()).unwrap())
        .collect()
}
