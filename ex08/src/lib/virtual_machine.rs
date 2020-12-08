use super::instruction::{Instruction, Operation};

pub struct VirtualMachine {
    instructions: Vec<Instruction>,
    accumulator: i32,
    offset: usize,
}

impl VirtualMachine {
    pub fn new(instructions: Vec<Instruction>) -> VirtualMachine {
        VirtualMachine {
            instructions,
            accumulator: 0,
            offset: 0,
        }
    }
    pub fn run(&mut self) -> Result<i32, Vec<usize>> {
        let mut instruction_traces: Vec<usize> = Vec::new();
        loop {
            if self.offset >= self.instructions.len() {
                println!("Succeeeded");
                return Ok(self.accumulator);
            }
            // TODO: refactor the instructions_trace to handle lot more values
            if instruction_traces.contains(&self.offset) {
                println!(
                    "Error: Infinite loop discovered at offset {} - accumulator value {}",
                    self.offset, self.accumulator
                );
                return Err(instruction_traces);
            }
            instruction_traces.push(self.offset);
            let c_instruction = self.instructions.get(self.offset);
            if c_instruction.is_none() {
                println!("Error: Found a None instruction!");
                return Err(instruction_traces);
            }
            let instruction = c_instruction.unwrap();
            let operation = instruction.get_operation();
            let argument = instruction.get_argument();
            match operation {
                Operation::Acc => {
                    self.accumulator += argument;
                    self.offset += 1
                }
                Operation::Jmp => {
                    if argument < 0 {
                        self.offset -= (argument * -1) as usize;
                    } else {
                        self.offset += argument as usize;
                    }
                }
                Operation::Nop => self.offset += 1,
            }
        }
    }
    pub fn clean(&mut self) {
        self.accumulator = 0;
        self.offset = 0;
    }
    pub fn debug_instruction(&self, instructions_trace: &Vec<usize>) -> Option<Vec<Instruction>> {
        // Reverse the trace
        let mut reversed_instructions = instructions_trace.iter().rev();
        let mut attempts = 0;
        loop {
            attempts += 1;
            let c_instruction_offset = reversed_instructions.next();
            if c_instruction_offset.is_none() {
                break;
            }
            let instruction_offset = *c_instruction_offset.unwrap();
            let instruction = self.instructions[instruction_offset];
            let mut operation = instruction.get_operation();
            // Brute force method: change everything, run another virtual machine, and check for error...
            operation = match &operation {
                Operation::Jmp => Operation::Nop,
                Operation::Nop => Operation::Jmp,
                _ => {
                    continue;
                }
            };
            let mut new_instruction_seq = self.instructions.clone();
            let old_instruction = new_instruction_seq[instruction_offset];
            new_instruction_seq[instruction_offset].replace_operation(operation);
            print!(
                "> Trying to replace instruction {:?} with {:?} at offset {}... ",
                old_instruction, new_instruction_seq[instruction_offset], instruction_offset
            );
            let mut t_virtual_machine = VirtualMachine::new(new_instruction_seq.clone());
            // If the current virtual machine runs...
            if let Ok(accumulator) = t_virtual_machine.run() {
                println!(
                    "Sequence {} *ok* (found {} as accumulator)",
                    attempts, accumulator
                );
                return Some(new_instruction_seq);
            }
        }

        None
    }
}
