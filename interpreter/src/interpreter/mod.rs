mod interpreter;
mod stack;

use crate::bytecode::ByteCode;
use interpreter::Interpreter;
use crate::VariableValue;

pub fn interpret(program: ByteCode) -> Result<Option<VariableValue>, String>  {
    let mut interpreter = Interpreter::new(program);
    interpreter.interpret()
}