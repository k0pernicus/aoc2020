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
    pub fn replace_argument(&mut self, new_argument: i32) {
        self.argument = new_argument;
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
