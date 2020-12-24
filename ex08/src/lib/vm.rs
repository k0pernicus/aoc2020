use std::convert::From;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}

impl From<&str> for Operation {
    fn from(word: &str) -> Self {
        match word.to_lowercase().as_str() {
            "acc" => Self::Acc,
            "jmp" => Self::Jmp,
            "nop" => Self::Nop,
            _ => Self::Nop,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    operation: Operation,
    argument: i32,
}

impl Instruction {
    pub fn get_operation(&self) -> Operation {
        return self.operation;
    }
    pub fn replace_operation(&mut self, new_operation: Operation) {
        self.operation = new_operation;
    }
    pub fn get_argument(&self) -> i32 {
        return self.argument;
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(raw_instruction: &str) -> Result<Instruction, Self::Err> {
        let raw_instruction_splited: Vec<&str> =
            raw_instruction.trim().split_whitespace().collect();
        if raw_instruction_splited.len() != 2 {
            println!("Raw instruction is not composed with two distinct parts...");
            return Err(());
        }
        let operation = Operation::from(raw_instruction_splited[0]);
        let argument = raw_instruction_splited[1].parse::<i32>().unwrap();
        Ok(Instruction {
            operation,
            argument,
        })
    }
}

pub struct VirtualMachine {
    instructions: Vec<Instruction>,
    accumulator: i32,
    offset: usize,
}

pub enum VirtualMachineError {
    DetectedLoop(Vec<usize>),
    NoneInstruction(),
}

impl VirtualMachine {
    pub fn new(instructions: Vec<Instruction>) -> VirtualMachine {
        VirtualMachine {
            instructions,
            accumulator: 0,
            offset: 0,
        }
    }
    pub fn run(&mut self) -> Result<i32, VirtualMachineError> {
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
                return Err(VirtualMachineError::DetectedLoop(instruction_traces));
            }
            instruction_traces.push(self.offset);
            let c_instruction = self.instructions.get(self.offset);
            if c_instruction.is_none() {
                println!("Error: Found a None instruction!");
                return Err(VirtualMachineError::NoneInstruction());
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
