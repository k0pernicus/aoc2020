use super::Instructions;
use bit_array::{BitArray, U10};

pub fn parse_program(
    program_instructions: Vec<String>,
) -> Result<(BitArray<u32, U10>, Instructions), ()> {
    Err(())
}
