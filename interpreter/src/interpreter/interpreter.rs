use std::collections::HashMap;
use super::stack::Stack;
use crate::bytecode::{instruction::Instruction, ByteCode};
use crate::config::{InstructionName, VariableValue};

struct InterpreterState {
    stack: Stack<VariableValue>,
    vars: HashMap<String, VariableValue>,
    instruction_pointer: usize,
}

impl InterpreterState {
    pub fn new() -> Self {
        let ret = InterpreterState {
            stack: Stack::new(),
            vars: HashMap::new(),
            instruction_pointer: 0,
        };
        ret
    }
    pub fn get_instruction_pointer(&self) -> usize {
        self.instruction_pointer
    }
    pub fn set_instruction_pointer(&mut self, instruction_pointer: usize) {
        self.instruction_pointer = instruction_pointer;
    }
    pub fn next(&mut self) {
        self.instruction_pointer += 1;
    }
    pub fn pop_value(&mut self) -> Result<VariableValue, String> {
        match self.stack.pop() {
            Some(v) => Ok(v),
            None => Err(format!(
                "Runtime error: unable to process instruction #{}: no value on stack",
                self.instruction_pointer + 1
            )),
        }
    }
    pub fn push_value(&mut self, v: VariableValue) {
        self.stack.push(v);
    }
    pub fn add_var(&mut self, variable_name: &str, value: VariableValue) {
        self.vars.insert(variable_name.to_string(), value);
    }
    pub fn read_var(&mut self, variable_name: &str) -> Result<VariableValue, String> {
        if !self.vars.contains_key(variable_name) {
            return Err(format!(
                "Runtime error: unable to get variable: doesn't exist, instruction#{}",
                self.instruction_pointer + 1
            ));
        }
        return Ok(*self.vars.get(variable_name).unwrap());
    }
}

pub struct Interpreter {
    bytecode: ByteCode,
}

impl Interpreter {
    pub fn new(bytecode: ByteCode) -> Self {
        Interpreter { bytecode }
    }
    pub fn interpret(&mut self) -> Result<Option<VariableValue>, String> {
        let mut interpreter_state = InterpreterState::new();
        loop {
            let instruction = match self.bytecode.get_instruction(interpreter_state.get_instruction_pointer() as usize) {
                Some(instruction) => instruction.clone(),
                None => return Ok(None),
            };
            let instruction_name = match Some(instruction.name) {
                Some(ref instruction_name) => instruction_name.clone(),
                None => return Err("Invalid instruction: invalid name".to_string()),
            };

            match instruction_name {
                InstructionName::LOAD => self.interpret_load_instruction(&mut interpreter_state, instruction),
                InstructionName::WRITE => self.interpret_write_instruction(&mut interpreter_state, instruction),
                InstructionName::READ => self.interpret_read_instruction(&mut interpreter_state, instruction),
                InstructionName::ADD => self.interpret_add_instruction(&mut interpreter_state),
                InstructionName::MULTIPLY => self.interpret_multiply_instruction(&mut interpreter_state),
                InstructionName::GREATER => self.interpret_greater_instruction(&mut interpreter_state),
                InstructionName::LESS => self.interpret_less_instruction(&mut interpreter_state),
                InstructionName::EQUAL => self.interpret_equal_instruction(&mut interpreter_state),
                InstructionName::DUP => self.interpret_dup_instruction(&mut interpreter_state),
                InstructionName::POP => self.interpret_pop_instruction(&mut interpreter_state),
                InstructionName::GOTO => self.interpret_goto_instruction(&mut interpreter_state, instruction),
                InstructionName::RETURN => {
                    return interpreter_state.pop_value().map(Some)
                }
            }?;
        }
    }

    fn interpret_load_instruction(&mut self, interpreter_state: &mut InterpreterState, instruction: Instruction) -> Result<(), String> {
        interpreter_state.push_value(instruction.arg.unwrap());
        interpreter_state.next();
        Ok(())
    }

    fn interpret_write_instruction(&mut self, interpreter_state: &mut InterpreterState, instruction: Instruction) -> Result<(), String> {
        match interpreter_state.pop_value() {
            Err(e) => return Err(e),
            Ok(value) => {
                interpreter_state.add_var(&instruction.variable.unwrap(), value);
                interpreter_state.next();
                Ok(())
            }
        }
    }

    fn interpret_read_instruction(&mut self, interpreter_state: &mut InterpreterState, instruction: Instruction) -> Result<(), String> {
        match interpreter_state.read_var(&instruction.variable.unwrap()) {
            Err(e) => return Err(e),
            Ok(value) => {
                interpreter_state.push_value(value);
                interpreter_state.next();
                Ok(())
            }
        }
    }

    fn interpret_add_instruction(&mut self, interpreter_state: &mut InterpreterState) -> Result<(), String> {
        let value1 = interpreter_state.pop_value()?;
        let value2= interpreter_state.pop_value()?;
        interpreter_state.push_value(value1 + value2);
        interpreter_state.next();
        Ok(())
    }

    fn interpret_multiply_instruction(&mut self, interpreter_state: &mut InterpreterState) -> Result<(), String> {
        let value1 = interpreter_state.pop_value()?;
        let value2= interpreter_state.pop_value()?;
        interpreter_state.push_value(value1 * value2);
        interpreter_state.next();
        Ok(())
    }

    fn interpret_greater_instruction(&mut self, interpreter_state: &mut InterpreterState) -> Result<(), String> {
        let value1 = interpreter_state.pop_value()?;
        let value2= interpreter_state.pop_value()?;
        interpreter_state.push_value((value1 > value2) as VariableValue);
        interpreter_state.next();
        Ok(())
    }

    fn interpret_less_instruction(&mut self, interpreter_state: &mut InterpreterState) -> Result<(), String> {
        let value1 = interpreter_state.pop_value()?;
        let value2= interpreter_state.pop_value()?;
        interpreter_state.push_value((value1 < value2) as VariableValue);
        interpreter_state.next();
        Ok(())
    }

    fn interpret_equal_instruction(&mut self, interpreter_state: &mut InterpreterState) -> Result<(), String> {
        let value1 = interpreter_state.pop_value()?;
        let value2= interpreter_state.pop_value()?;
        interpreter_state.push_value((value1 == value2) as VariableValue);
        interpreter_state.next();
        Ok(())
    }

    fn interpret_dup_instruction(&mut self, interpreter_state: &mut InterpreterState) -> Result<(), String> {
        let value = interpreter_state.pop_value()?;
        interpreter_state.push_value(value);
        interpreter_state.push_value(value);
        interpreter_state.next();
        Ok(())
    }

    fn interpret_pop_instruction(&mut self, interpreter_state: &mut InterpreterState) -> Result<(), String> {
        interpreter_state.pop_value()?;
        interpreter_state.next();
        Ok(())
    }

    fn interpret_goto_instruction(&mut self, interpreter_state: &mut InterpreterState, instruction: Instruction) -> Result<(), String> {
        let label = instruction.variable.unwrap();
        let label_instruction_number = match self.bytecode.get_label(&label) {
            Ok(number) => number.clone(),
            Err(e) => return Err(e)
        };
        let value = interpreter_state.pop_value()?;
        if value == 0 {
            interpreter_state.next();
        } else {
            interpreter_state.set_instruction_pointer(label_instruction_number as usize)
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytecode::instruction::Instruction;

    #[test]
    fn test_interpret_load() {
        let load_value: VariableValue = 10;
        let mut interpreter_state = InterpreterState::new();
        let load_instruction = Instruction {
            name: InstructionName::LOAD,
            arg: Some(load_value),
            variable: None,
        };

        let mut interpreter = Interpreter::new(ByteCode::new());

        let result = Interpreter::interpret_load_instruction(&mut interpreter, &mut interpreter_state, load_instruction);

        assert!(result.is_ok());
        let on_stack = interpreter_state.pop_value();
        assert!(on_stack.is_ok());
        assert_eq!(on_stack.ok().unwrap(), load_value);
        assert_eq!(interpreter_state.instruction_pointer, 0x1);
    }

    #[test]
    fn test_interpret_write() {
        let load_value: VariableValue = 10;
        let var_name = "x";
        let mut interpreter_state = InterpreterState::new();
        interpreter_state.push_value(load_value);
        interpreter_state.next();
        let load_instruction = Instruction {
            name: InstructionName::WRITE,
            arg: None,
            variable: Some(var_name.to_string()),
        };

        let mut interpreter = Interpreter::new(ByteCode::new());

        let result = Interpreter::interpret_write_instruction(&mut interpreter, &mut interpreter_state, load_instruction);

        assert!(result.is_ok());
        assert_eq!(*interpreter_state.vars.get(var_name).unwrap(), load_value);
        assert_eq!(interpreter_state.instruction_pointer, 0x2);
    }

    #[test]
    fn test_interpret_write_with_empty_stack() {
        let var_name = "x";
        let mut interpreter_state = InterpreterState::new();
        let load_instruction = Instruction {
            name: InstructionName::WRITE,
            arg: None,
            variable: Some(var_name.to_string()),
        };

        let mut interpreter = Interpreter::new(ByteCode::new());

        let result = Interpreter::interpret_write_instruction(&mut interpreter, &mut interpreter_state, load_instruction);

        assert_eq!(
            result,
            Err("Runtime error: unable to process instruction #1: no value on stack".to_string())
        )
    }

    #[test]
    fn test_interpret_read() {
        let load_value: VariableValue = 10;
        let var_name = "x";
        let mut interpreter_state = InterpreterState::new();
        interpreter_state.vars.insert(var_name.to_string(), load_value);
        let load_instruction = Instruction {
            name: InstructionName::READ,
            arg: None,
            variable: Some(var_name.to_string()),
        };

        let mut interpreter = Interpreter::new(ByteCode::new());

        let result = Interpreter::interpret_read_instruction(&mut interpreter, &mut interpreter_state, load_instruction);

        assert!(result.is_ok());
        let on_stack = interpreter_state.pop_value();
        assert!(on_stack.is_ok());
        assert_eq!(on_stack.ok().unwrap(), load_value);
        assert_eq!(interpreter_state.instruction_pointer, 0x1);
    }

    #[test]
    fn test_interpret_read_with_not_existing_variable() {
        let var_name = "x";
        let mut interpreter_state = InterpreterState::new();
        let load_instruction = Instruction {
            name: InstructionName::READ,
            arg: None,
            variable: Some(var_name.to_string()),
        };

        let mut interpreter = Interpreter::new(ByteCode::new());

        let result = Interpreter::interpret_read_instruction(&mut interpreter, &mut interpreter_state, load_instruction);

        assert_eq!(
            result,
            Err("Runtime error: unable to get variable: doesn't exist, instruction#1".to_string())
        )
    }

    #[test]
    fn test_interpret_add() {
        let load_value1: VariableValue = 10;
        let load_value2: VariableValue = 10;
        let mut interpreter_state = InterpreterState::new();
        interpreter_state.push_value(load_value1);
        interpreter_state.push_value(load_value2);

        let mut interpreter = Interpreter::new(ByteCode::new());

        let result = Interpreter::interpret_add_instruction(&mut interpreter, &mut interpreter_state);

        assert!(result.is_ok());
        let on_stack = interpreter_state.pop_value();
        assert!(on_stack.is_ok());
        assert_eq!(on_stack.ok().unwrap(), load_value1 + load_value2);
        assert_eq!(interpreter_state.instruction_pointer, 0x1);
    }

    #[test]
    fn test_interpret_add_with_bad_stack_state() {
        let load_value1: VariableValue = 10;
        let mut interpreter_state = InterpreterState::new();
        interpreter_state.push_value(load_value1);

        let mut interpreter = Interpreter::new(ByteCode::new());

        let result = Interpreter::interpret_add_instruction(&mut interpreter, &mut interpreter_state);

        assert_eq!(
            result,
            Err("Runtime error: unable to process instruction #1: no value on stack".to_string())
        )
    }

    #[test]
    fn test_interpret_multiply() {
        let load_value1: VariableValue = 10;
        let load_value2: VariableValue = 10;
        let mut interpreter_state = InterpreterState::new();
        interpreter_state.push_value(load_value1);
        interpreter_state.push_value(load_value2);

        let mut interpreter = Interpreter::new(ByteCode::new());

        let result = Interpreter::interpret_multiply_instruction(&mut interpreter, &mut interpreter_state);

        assert!(result.is_ok());
        let on_stack = interpreter_state.pop_value();
        assert!(on_stack.is_ok());
        assert_eq!(on_stack.ok().unwrap(), load_value1 * load_value2);
        assert_eq!(interpreter_state.instruction_pointer, 0x1);
    }

    #[test]
    fn test_interpret_multiply_with_bad_stack_state() {
        let load_value1: VariableValue = 10;
        let mut interpreter_state = InterpreterState::new();
        interpreter_state.push_value(load_value1);

        let mut interpreter = Interpreter::new(ByteCode::new());

        let result = Interpreter::interpret_multiply_instruction(&mut interpreter, &mut interpreter_state);

        assert_eq!(
            result,
            Err("Runtime error: unable to process instruction #1: no value on stack".to_string())
        )
    }

    #[test]
    fn test_interpret_greater() {
        let load_value1: VariableValue = 10;
        let load_value2: VariableValue = 20;
        let mut interpreter_state = InterpreterState::new();
        interpreter_state.push_value(load_value1);
        interpreter_state.push_value(load_value2);

        let mut interpreter = Interpreter::new(ByteCode::new());

        let result = Interpreter::interpret_greater_instruction(&mut interpreter, &mut interpreter_state);

        assert!(result.is_ok());
        let on_stack = interpreter_state.pop_value();
        assert!(on_stack.is_ok());
        assert_eq!(on_stack.ok().unwrap(), 1);
        assert_eq!(interpreter_state.instruction_pointer, 0x1);
    }

    #[test]
    fn test_interpret_greater_with_bad_stack_state() {
        let load_value1: VariableValue = 10;
        let mut interpreter_state = InterpreterState::new();
        interpreter_state.push_value(load_value1);

        let mut interpreter = Interpreter::new(ByteCode::new());

        let result = Interpreter::interpret_greater_instruction(&mut interpreter, &mut interpreter_state);

        assert_eq!(
            result,
            Err("Runtime error: unable to process instruction #1: no value on stack".to_string())
        )
    }

    #[test]
    fn test_interpret_less() {
        let load_value1: VariableValue = 10;
        let load_value2: VariableValue = 20;
        let mut interpreter_state = InterpreterState::new();
        interpreter_state.push_value(load_value1);
        interpreter_state.push_value(load_value2);

        let mut interpreter = Interpreter::new(ByteCode::new());

        let result = Interpreter::interpret_less_instruction(&mut interpreter, &mut interpreter_state);

        assert!(result.is_ok());
        let on_stack = interpreter_state.pop_value();
        assert!(on_stack.is_ok());
        assert_eq!(on_stack.ok().unwrap(), 0);
        assert_eq!(interpreter_state.instruction_pointer, 0x1);
    }

    #[test]
    fn test_interpret_less_with_bad_stack_state() {
        let load_value1: VariableValue = 10;
        let mut interpreter_state = InterpreterState::new();
        interpreter_state.push_value(load_value1);

        let mut interpreter = Interpreter::new(ByteCode::new());

        let result = Interpreter::interpret_less_instruction(&mut interpreter, &mut interpreter_state);

        assert_eq!(
            result,
            Err("Runtime error: unable to process instruction #1: no value on stack".to_string())
        )
    }

    #[test]
    fn test_interpret_equal() {
        let load_value1: VariableValue = 10;
        let load_value2: VariableValue = 10;
        let mut interpreter_state = InterpreterState::new();
        interpreter_state.push_value(load_value1);
        interpreter_state.push_value(load_value2);

        let mut interpreter = Interpreter::new(ByteCode::new());

        let result = Interpreter::interpret_equal_instruction(&mut interpreter, &mut interpreter_state);

        assert!(result.is_ok());
        let on_stack = interpreter_state.pop_value();
        assert!(on_stack.is_ok());
        assert_eq!(on_stack.ok().unwrap(), 1);
        assert_eq!(interpreter_state.instruction_pointer, 0x1);
    }

    #[test]
    fn test_interpret_equal_with_bad_stack_state() {
        let load_value1: VariableValue = 10;
        let mut interpreter_state = InterpreterState::new();
        interpreter_state.push_value(load_value1);

        let mut interpreter = Interpreter::new(ByteCode::new());

        let result = Interpreter::interpret_equal_instruction(&mut interpreter, &mut interpreter_state);

        assert_eq!(
            result,
            Err("Runtime error: unable to process instruction #1: no value on stack".to_string())
        )
    }

    #[test]
    fn test_interpret_dup() {
        let load_value: VariableValue = 10;
        let mut interpreter_state = InterpreterState::new();
        interpreter_state.push_value(load_value);

        let mut interpreter = Interpreter::new(ByteCode::new());

        let result = Interpreter::interpret_dup_instruction(&mut interpreter, &mut interpreter_state);

        assert!(result.is_ok());
        let on_stack = interpreter_state.pop_value();
        assert!(on_stack.is_ok());
        assert_eq!(on_stack.ok().unwrap(), 10);
        let on_stack = interpreter_state.pop_value();
        assert!(on_stack.is_ok());
        assert_eq!(on_stack.ok().unwrap(), 10);
        assert_eq!(interpreter_state.instruction_pointer, 0x1);
    }

    #[test]
    fn test_interpret_dup_with_bad_stack_state() {
        let mut interpreter_state = InterpreterState::new();

        let mut interpreter = Interpreter::new(ByteCode::new());

        let result = Interpreter::interpret_dup_instruction(&mut interpreter, &mut interpreter_state);

        assert_eq!(
            result,
            Err("Runtime error: unable to process instruction #1: no value on stack".to_string())
        )
    }

    #[test]
    fn test_interpret_pop() {
        let load_value: VariableValue = 10;
        let mut interpreter_state = InterpreterState::new();
        interpreter_state.push_value(load_value);

        let mut interpreter = Interpreter::new(ByteCode::new());

        let result = Interpreter::interpret_pop_instruction(&mut interpreter, &mut interpreter_state);

        assert!(result.is_ok());
        let on_stack = interpreter_state.pop_value();
        assert_eq!(
            on_stack,
            Err("Runtime error: unable to process instruction #2: no value on stack".to_string())
        )
    }

    #[test]
    fn test_interpret_pop_with_bad_stack_state() {
        let mut interpreter_state = InterpreterState::new();

        let mut interpreter = Interpreter::new(ByteCode::new());

        let result = Interpreter::interpret_pop_instruction(&mut interpreter, &mut interpreter_state);

        assert_eq!(
            result,
            Err("Runtime error: unable to process instruction #1: no value on stack".to_string())
        )
    }
}