pub type VariableValue = u16;
pub type MaxInstructionNumber = u16;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InstructionName {
    LOAD,
    WRITE,
    READ,
    ADD,
    MULTIPLY,
    RETURN,
    EQUAL,
    GREATER,
    LESS,
    DUP,
    POP,
    GOTO,
}