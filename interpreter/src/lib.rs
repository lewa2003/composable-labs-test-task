mod bytecode;
mod interpreter;
mod config;

use bytecode::ByteCode;
use interpreter::interpret as int;
use crate::config::VariableValue;

pub fn interpret(source_file: &str) -> Result<Option<VariableValue>, String> {
    let byte_code = ByteCode::parse_file(source_file)?;
    int(byte_code)
}