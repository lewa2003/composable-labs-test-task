use crate::config::{InstructionName, VariableValue};
use lazy_static::lazy_static;
use regex::Regex;
use crate::ByteCode;

#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
    pub name: InstructionName,
    pub arg: Option<VariableValue>,
    pub variable: Option<String>,
}

impl Instruction {

    pub fn new(instruction_name: &str, args: Vec<String>) -> Result<Self, String>{
        let instruction = match instruction_name {
            "LOAD_VAL" => Instruction::create_load_instruction(args),
            "WRITE_VAR" => Instruction::create_write_instruction(args),
            "READ_VAR" => Instruction::create_read_instruction(args),
            "ADD" => Instruction::create_add_instruction(args),
            "MULTIPLY" => Instruction::create_multiply_instruction(args),
            "RETURN_VALUE" => Instruction::create_return_instruction(args),
            "GREATER" => Instruction::create_greater_instruction(args),
            "LESS" => Instruction::create_less_instruction(args),
            "EQUAL" => Instruction::create_equal_instruction(args),
            "DUP" => Instruction::create_dup_instruction(args),
            "POP" => Instruction::create_pop_instruction(args),
            "GOTO" => Instruction::create_goto_instruction(args),
            _other => Err(format!("Unknown instruction: {}", instruction_name)),
        };
        return instruction;
    }

    fn create_load_instruction(args: Vec<String>) -> Result<Self, String> {
        if args.len() != 1 {
            return Err(format!("Error creating load instruction: expected 1 argument, got {}", args.len()))
        }

        let arg = match args.get(0).unwrap().parse::<VariableValue>() {
            Ok(v) => v,
            Err(e) => return Err(format!("Error creating load instruction: {}", e)),
        };

        let instr = Instruction {
            name: InstructionName::LOAD,
            arg: Some(arg),
            variable: None,
        };
        Ok(instr)
    }

    fn create_write_instruction(args: Vec<String>) -> Result<Self, String> {
        if args.len() != 1 {
            return Err(format!("Error creating write instruction: expected 1 argument, got {}", args.len()))
        }

        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").expect("Invalid regex");
        }
        let arg = args.get(0).unwrap().to_string();
        if !RE.is_match(&arg) {
            return Err(format!("Invalid variable name {}", arg));
        }

        let instr = Instruction {
            name: InstructionName::WRITE,
            arg: None,
            variable: Some(arg),
        };
        Ok(instr)
    }

    fn create_read_instruction(args: Vec<String>) -> Result<Self, String> {
        if args.len() != 1 {
            return Err(format!("Error creating read instruction: expected 1 argument, got {}", args.len()))
        }
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").expect("Invalid regex");
        }
        let arg = args.get(0).unwrap().to_string();
        if !RE.is_match(&arg) {
            return Err(format!("Invalid variable name {}", arg));
        }

        let instr = Instruction {
            name: InstructionName::READ,
            arg: None,
            variable: Some(arg),
        };
        Ok(instr)
    }

    fn create_add_instruction(args: Vec<String>) -> Result<Self, String> {
        if !args.is_empty() {
            return Err(format!("Error creating add instruction: expected 0 argument, got {}", args.len()))
        }

        let instr = Instruction {
            name: InstructionName::ADD,
            arg: None,
            variable: None,
        };
        Ok(instr)
    }

    fn create_multiply_instruction(args: Vec<String>) -> Result<Self, String> {
        if !args.is_empty() {
            return Err(format!("Error creating multiply instruction: expected 0 argument, got {}", args.len()))
        }

        let instr = Instruction {
            name: InstructionName::MULTIPLY,
            arg: None,
            variable: None,
        };
        Ok(instr)
    }

    fn create_return_instruction(args: Vec<String>) -> Result<Self, String> {
        if !args.is_empty() {
            return Err(format!("Error creating return instruction: expected 0 argument, got {}", args.len()))
        }

        let instr = Instruction {
            name: InstructionName::RETURN,
            arg: None,
            variable: None,
        };
        Ok(instr)
    }

    fn create_greater_instruction(args: Vec<String>) -> Result<Self, String> {
        if !args.is_empty() {
            return Err(format!("Error creating greater instruction: expected 0 argument, got {}", args.len()))
        }

        let instr = Instruction {
            name: InstructionName::GREATER,
            arg: None,
            variable: None,
        };
        Ok(instr)
    }

    fn create_less_instruction(args: Vec<String>) -> Result<Self, String> {
        if !args.is_empty() {
            return Err(format!("Error creating less instruction: expected 0 argument, got {}", args.len()))
        }

        let instr = Instruction {
            name: InstructionName::LESS,
            arg: None,
            variable: None,
        };
        Ok(instr)
    }

    fn create_equal_instruction(args: Vec<String>) -> Result<Self, String> {
        if !args.is_empty() {
            return Err(format!("Error creating equal instruction: expected 0 argument, got {}", args.len()))
        }

        let instr = Instruction {
            name: InstructionName::EQUAL,
            arg: None,
            variable: None,
        };
        Ok(instr)
    }

    fn create_dup_instruction(args: Vec<String>) -> Result<Self, String> {
        if !args.is_empty() {
            return Err(format!("Error creating dup instruction: expected 0 argument, got {}", args.len()))
        }

        let instr = Instruction {
            name: InstructionName::DUP,
            arg: None,
            variable: None,
        };
        Ok(instr)
    }

    fn create_pop_instruction(args: Vec<String>) -> Result<Self, String> {
        if !args.is_empty() {
            return Err(format!("Error creating pop instruction: expected 0 argument, got {}", args.len()))
        }

        let instr = Instruction {
            name: InstructionName::POP,
            arg: None,
            variable: None,
        };
        Ok(instr)
    }

    fn create_goto_instruction(args: Vec<String>) -> Result<Self, String> {
        if args.len() != 1 {
            return Err(format!("Error creating goto instruction: expected 1 argument, got {}", args.len()))
        }
        let arg = args.get(0).unwrap().to_string();
        if !ByteCode::is_label(&arg) {
            return Err(format!("Invalid label name: {}", arg));
        }

        let instr = Instruction {
            name: InstructionName::GOTO,
            arg: None,
            variable: Some(arg),
        };
        Ok(instr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_load_instruction() {
        let instruction_name = "LOAD_VAL";
        let mut args= Vec::new();
        args.push("1".to_string());
        let result = Instruction::new(instruction_name, args);
        assert!(result.is_ok());
        let instruction = result.ok().unwrap();
        assert_eq!(instruction.name, InstructionName::LOAD);
        assert_eq!(instruction.variable, None);
        assert_eq!(instruction.arg, Some(1));
    }

    #[test]
    fn create_load_instruction_with_more_than_one_arg() {
        let instruction_name = "LOAD_VAL";
        let mut args= Vec::new();
        args.push("1".to_string());
        args.push("2".to_string());
        let result = Instruction::new(instruction_name, args);
        assert_eq!(
            result,
            Err("Error creating load instruction: expected 1 argument, got 2".to_string())
        );
    }

    #[test]
    fn create_load_instruction_with_zero_args() {
        let instruction_name = "LOAD_VAL";
        let args= Vec::new();
        let result = Instruction::new(instruction_name, args);
        assert_eq!(
            result,
            Err("Error creating load instruction: expected 1 argument, got 0".to_string())
        );
    }

    #[test]
    fn create_write_instruction() {
        let instruction_name = "WRITE_VAR";
        let mut args= Vec::new();
        args.push("x".to_string());
        let result = Instruction::new(instruction_name, args);
        assert!(result.is_ok());
        let instruction = result.ok().unwrap();
        assert_eq!(instruction.name, InstructionName::WRITE);
        assert_eq!(instruction.variable, Some("x".to_string()));
        assert_eq!(instruction.arg, None);
    }

    #[test]
    fn create_write_instruction_with_more_than_one_arg() {
        let instruction_name = "WRITE_VAR";
        let mut args= Vec::new();
        args.push("x".to_string());
        args.push("y".to_string());
        let result = Instruction::new(instruction_name, args);
        assert_eq!(
            result,
            Err("Error creating write instruction: expected 1 argument, got 2".to_string())
        );
    }

    #[test]
    fn create_write_instruction_with_zero_args() {
        let instruction_name = "WRITE_VAR";
        let args= Vec::new();
        let result = Instruction::new(instruction_name, args);
        assert_eq!(
            result,
            Err("Error creating write instruction: expected 1 argument, got 0".to_string())
        );
    }

    #[test]
    fn create_read_instruction() {
        let instruction_name = "READ_VAR";
        let mut args= Vec::new();
        args.push("x".to_string());
        let result = Instruction::new(instruction_name, args);
        assert!(result.is_ok());
        let instruction = result.ok().unwrap();
        assert_eq!(instruction.name, InstructionName::READ);
        assert_eq!(instruction.variable, Some("x".to_string()));
        assert_eq!(instruction.arg, None);
    }

    #[test]
    fn create_read_instruction_with_more_than_one_arg() {
        let instruction_name = "READ_VAR";
        let mut args= Vec::new();
        args.push("x".to_string());
        args.push("y".to_string());
        let result = Instruction::new(instruction_name, args);
        assert_eq!(
            result,
            Err("Error creating read instruction: expected 1 argument, got 2".to_string())
        );
    }

    #[test]
    fn create_read_instruction_with_zero_args() {
        let instruction_name = "READ_VAR";
        let args= Vec::new();
        let result = Instruction::new(instruction_name, args);
        assert_eq!(
            result,
            Err("Error creating read instruction: expected 1 argument, got 0".to_string())
        );
    }

    #[test]
    fn create_add_instruction() {
        let instruction_name = "ADD";
        let args= Vec::new();
        let result = Instruction::new(instruction_name, args);
        assert!(result.is_ok());
        let instruction = result.ok().unwrap();
        assert_eq!(instruction.name, InstructionName::ADD);
        assert_eq!(instruction.variable, None);
        assert_eq!(instruction.arg, None);
    }

    #[test]
    fn create_add_instruction_with_more_than_zero_arg() {
        let instruction_name = "ADD";
        let mut args= Vec::new();
        args.push("y".to_string());
        let result = Instruction::new(instruction_name, args);
        assert_eq!(
            result,
            Err("Error creating add instruction: expected 0 argument, got 1".to_string())
        );
    }

    #[test]
    fn create_multiply_instruction() {
        let instruction_name = "MULTIPLY";
        let args= Vec::new();
        let result = Instruction::new(instruction_name, args);
        assert!(result.is_ok());
        let instruction = result.ok().unwrap();
        assert_eq!(instruction.name, InstructionName::MULTIPLY);
        assert_eq!(instruction.variable, None);
        assert_eq!(instruction.arg, None);
    }

    #[test]
    fn create_multiply_instruction_with_more_than_zero_arg() {
        let instruction_name = "MULTIPLY";
        let mut args= Vec::new();
        args.push("y".to_string());
        let result = Instruction::new(instruction_name, args);
        assert_eq!(
            result,
            Err("Error creating multiply instruction: expected 0 argument, got 1".to_string())
        );
    }

    #[test]
    fn create_return_instruction() {
        let instruction_name = "RETURN_VALUE";
        let args= Vec::new();
        let result = Instruction::new(instruction_name, args);
        assert!(result.is_ok());
        let instruction = result.ok().unwrap();
        assert_eq!(instruction.name, InstructionName::RETURN);
        assert_eq!(instruction.variable, None);
        assert_eq!(instruction.arg, None);
    }

    #[test]
    fn create_return_instruction_with_more_than_zero_arg() {
        let instruction_name = "RETURN_VALUE";
        let mut args= Vec::new();
        args.push("y".to_string());
        let result = Instruction::new(instruction_name, args);
        assert_eq!(
            result,
            Err("Error creating return instruction: expected 0 argument, got 1".to_string())
        );
    }

    #[test]
    fn create_unknown_instruction() {
        let instruction_name = "INSTRUCTION_NAME";
        let mut args= Vec::new();
        args.push("y".to_string());
        let result = Instruction::new(instruction_name, args);
        assert_eq!(
            result,
            Err("Unknown instruction: INSTRUCTION_NAME".to_string())
        );
    }

    #[test]
    fn create_greater_instruction() {
        let instruction_name = "GREATER";
        let args= Vec::new();
        let result = Instruction::new(instruction_name, args);
        assert!(result.is_ok());
        let instruction = result.ok().unwrap();
        assert_eq!(instruction.name, InstructionName::GREATER);
        assert_eq!(instruction.variable, None);
        assert_eq!(instruction.arg, None);
    }

    #[test]
    fn create_greater_instruction_with_more_than_zero_arg() {
        let instruction_name = "GREATER";
        let mut args= Vec::new();
        args.push("y".to_string());
        let result = Instruction::new(instruction_name, args);
        assert_eq!(
            result,
            Err("Error creating greater instruction: expected 0 argument, got 1".to_string())
        );
    }

    #[test]
    fn create_less_instruction() {
        let instruction_name = "LESS";
        let args= Vec::new();
        let result = Instruction::new(instruction_name, args);
        assert!(result.is_ok());
        let instruction = result.ok().unwrap();
        assert_eq!(instruction.name, InstructionName::LESS);
        assert_eq!(instruction.variable, None);
        assert_eq!(instruction.arg, None);
    }

    #[test]
    fn create_less_instruction_with_more_than_zero_arg() {
        let instruction_name = "LESS";
        let mut args= Vec::new();
        args.push("y".to_string());
        let result = Instruction::new(instruction_name, args);
        assert_eq!(
            result,
            Err("Error creating less instruction: expected 0 argument, got 1".to_string())
        );
    }

    #[test]
    fn create_equal_instruction() {
        let instruction_name = "EQUAL";
        let args= Vec::new();
        let result = Instruction::new(instruction_name, args);
        assert!(result.is_ok());
        let instruction = result.ok().unwrap();
        assert_eq!(instruction.name, InstructionName::EQUAL);
        assert_eq!(instruction.variable, None);
        assert_eq!(instruction.arg, None);
    }

    #[test]
    fn create_equal_instruction_with_more_than_zero_arg() {
        let instruction_name = "EQUAL";
        let mut args= Vec::new();
        args.push("y".to_string());
        let result = Instruction::new(instruction_name, args);
        assert_eq!(
            result,
            Err("Error creating equal instruction: expected 0 argument, got 1".to_string())
        );
    }

    #[test]
    fn create_dup_instruction() {
        let instruction_name = "DUP";
        let args= Vec::new();
        let result = Instruction::new(instruction_name, args);
        assert!(result.is_ok());
        let instruction = result.ok().unwrap();
        assert_eq!(instruction.name, InstructionName::DUP);
        assert_eq!(instruction.variable, None);
        assert_eq!(instruction.arg, None);
    }

    #[test]
    fn create_dup_instruction_with_more_than_zero_arg() {
        let instruction_name = "DUP";
        let mut args= Vec::new();
        args.push("y".to_string());
        let result = Instruction::new(instruction_name, args);
        assert_eq!(
            result,
            Err("Error creating dup instruction: expected 0 argument, got 1".to_string())
        );
    }

    #[test]
    fn create_pop_instruction() {
        let instruction_name = "POP";
        let args= Vec::new();
        let result = Instruction::new(instruction_name, args);
        assert!(result.is_ok());
        let instruction = result.ok().unwrap();
        assert_eq!(instruction.name, InstructionName::POP);
        assert_eq!(instruction.variable, None);
        assert_eq!(instruction.arg, None);
    }

    #[test]
    fn create_pop_instruction_with_more_than_zero_arg() {
        let instruction_name = "POP";
        let mut args= Vec::new();
        args.push("y".to_string());
        let result = Instruction::new(instruction_name, args);
        assert_eq!(
            result,
            Err("Error creating pop instruction: expected 0 argument, got 1".to_string())
        );
    }

    #[test]
    fn create_goto_instruction() {
        let instruction_name = "GOTO";
        let mut args= Vec::new();
        args.push(".label".to_string());
        let result = Instruction::new(instruction_name, args);
        assert!(result.is_ok());
        let instruction = result.ok().unwrap();
        assert_eq!(instruction.name, InstructionName::GOTO);
        assert_eq!(instruction.variable, Some(".label".to_string()));
        assert_eq!(instruction.arg, None);
    }

    #[test]
    fn create_goto_instruction_with_more_than_one_arg() {
        let instruction_name = "GOTO";
        let mut args= Vec::new();
        args.push("x".to_string());
        args.push("y".to_string());
        let result = Instruction::new(instruction_name, args);
        assert_eq!(
            result,
            Err("Error creating goto instruction: expected 1 argument, got 2".to_string())
        );
    }

    #[test]
    fn create_goto_instruction_with_zero_args() {
        let instruction_name = "GOTO";
        let args= Vec::new();
        let result = Instruction::new(instruction_name, args);
        assert_eq!(
            result,
            Err("Error creating goto instruction: expected 1 argument, got 0".to_string())
        );
    }

    #[test]
    fn create_goto_instruction_with_incorrect_label_name() {
        let instruction_name = "GOTO";
        let mut args= Vec::new();
        args.push("label.".to_string());
        let result = Instruction::new(instruction_name, args);
        assert_eq!(
            result,
            Err("Invalid label name: label.".to_string())
        );
    }
}