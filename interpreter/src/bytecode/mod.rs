pub mod instruction;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use instruction::Instruction;
use lazy_static::lazy_static;
use regex::Regex;
use crate::config::MaxInstructionNumber;

#[derive(Debug, PartialEq)]
pub struct ByteCode {
    instructions: Vec<Instruction>,
    labels: HashMap<String, MaxInstructionNumber>,
}

impl ByteCode {
    pub fn new() -> Self {
        ByteCode {
            instructions: Vec::new(),
            labels: HashMap::new(),
        }
    }

    pub fn parse_file(source_file: &str) -> Result<Self, String> {
        let file = match File::open(source_file) {
            Ok(file) => file,
            Err(e) => return Err(format!("Unable to open file: {}", e)),
        };

        let mut reader = BufReader::new(file);
        ByteCode::parse_instructions(&mut reader)
    }

    pub fn get_instruction(&self, index: usize) -> Option<&Instruction> {
        self.instructions.get(index)
    }

    pub fn get_label(&self, label_name: &str) -> Result<MaxInstructionNumber, String> {
        if !self.labels.contains_key(label_name) {
            return Err(format!("Label with name: {} doesn't exist", label_name))
        }
        Ok(*self.labels.get(label_name).unwrap())
    }

    fn parse_instructions<R: BufRead>(reader: &mut R) -> Result<Self, String> {
        let mut program = ByteCode::new();
        let mut instruction_number = 0;
        for (index, line) in reader.lines().enumerate() {
            let ln = match line {
                Ok(line) => line,
                Err(e) => return Err(format!("Error reading line #{}: {}", index + 1, e)),
            };
            if ln.is_empty() {
                continue;
            }
            let s_split = ln
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            if ByteCode::is_label(&s_split[0]) {
                let label = program.parse_label(&s_split[0], instruction_number);
                if let Err(e) = label {
                    return Err(format!("Unable to parse line #{}: {}", index + 1, e));
                }
            } else {
                let instruction = Instruction::new(&s_split[0], s_split[1..].to_vec());
                if let Err(e) = instruction {
                    return Err(format!("Unable to parse line #{}: {}", index + 1, e));
                } else {
                    program.instructions.push(instruction.unwrap())
                }
                instruction_number += 1;
            }
        }
        if program.instructions.is_empty() {
            return Err("Empty program".to_string());
        }
        return Ok(program);
    }

    pub fn is_label(label: &str) -> bool {
        lazy_static! {
            static ref LABEL_RE: Regex =
                Regex::new(r"^\.[a-zA-Z_0-9][a-zA-Z0-9_]*").expect("Invalid regex");
        }
        LABEL_RE.is_match(label)
    }

    fn parse_label(&mut self, label_name: &str, instruction_number: u16) -> Result<(), String>{
        if self.labels.contains_key(label_name) {
            Err(format!("duplicated label: {}", label_name))?;
        }
        self.labels.insert(label_name.to_string(), instruction_number);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::VariableValue;
    use super::*;

    #[test]
    fn create_load_instruction() {
        let code = "LOAD_VAL 1";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::parse_instructions(&mut reader);
        assert!(result.is_ok());
        let bytecode = result.ok().unwrap();
        assert_eq!(bytecode.instructions.len(), 1);
    }

    #[test]
    fn create_two_write_instructions() {
        let code = "WRITE_VAR x\nWRITE_VAR y";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::parse_instructions(&mut reader);
        assert!(result.is_ok());
        let bytecode = result.ok().unwrap();
        assert_eq!(bytecode.instructions.len(), 2);
    }

    #[test]
    fn create_read_invalid_arg_num_instruction() {
        let code = "READ_VAR x y";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::parse_instructions(&mut reader);
        assert_eq!(
            result,
            Err("Unable to parse line #1: Error creating read instruction: expected 1 argument, got 2".to_string())
        );
    }

    #[test]
    fn create_empty_program() {
        let code = "";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::parse_instructions(&mut reader);
        assert_eq!(
            result,
            Err("Empty program".to_string())
        );
    }

    #[test]
    fn create_label_instruction() {
        let code = "LOAD_VAL 1\nREAD_VAR x\n.label\nGOTO .label";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::parse_instructions(&mut reader);
        assert!(result.is_ok());
        let bytecode = result.ok().unwrap();
        assert_eq!(*bytecode.labels.get(".label").unwrap(), 2 as VariableValue);
        assert_eq!(bytecode.instructions.len(), 3)
    }
}