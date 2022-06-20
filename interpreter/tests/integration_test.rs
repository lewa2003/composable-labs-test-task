use interpreter_app::interpret;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
pub fn test_empty_program() {
    let mut file = NamedTempFile::new().expect("Unable to create temp file");
    let program = "";
    write!(file, "{}", program).expect("Unable to write to temp file");

    let result = interpret(
        file.path()
            .to_str()
            .expect("Unable to convert temp file path to string"),
    );
    assert_eq!(result, Err("Empty program".to_string()));
}

#[test]
pub fn test_empty_stack() {
    let mut file = NamedTempFile::new().expect("Unable to create temp file");
    let program = "WRITE_VAR x";
    write!(file, "{}", program).expect("Unable to write to temp file");

    let result = interpret(
        file.path()
            .to_str()
            .expect("Unable to convert temp file path to string"),
    );
    assert_eq!(
        result,
        Err("Runtime error: unable to process instruction #1: no value on stack".to_string())
    );
}

#[test]
pub fn test_bad_instruction() {
    let mut file = NamedTempFile::new().expect("Unable to create temp file");
    let program =
        "LOAD_VAL 10
        WRITE_VAR x
        LOAD_VAL
        WRITE_VAR y";
    write!(file, "{}", program).expect("Unable to write to temp file");

    let result = interpret(
        file.path()
            .to_str()
            .expect("Unable to convert temp file path to string"),
    );
    assert_eq!(
        result,
        Err("Unable to parse line #3: Error creating load instruction: expected 1 argument, got 0".to_string())
    );
}

#[test]
pub fn test_duplicated_labels() {
    let mut file = NamedTempFile::new().expect("Unable to create temp file");
    let program =
        ".label
        .label
        GOTO .label";
    write!(file, "{}", program).expect("Unable to write to temp file");

    let result = interpret(
        file.path()
            .to_str()
            .expect("Unable to convert temp file path to string"),
    );
    assert_eq!(
        result,
        Err("Unable to parse line #2: duplicated label: .label".to_string())
    );
}

#[test]
pub fn test_non_existent_label() {
    let mut file = NamedTempFile::new().expect("Unable to create temp file");
    let program = ".label\nGOTO .newLabel";
    write!(file, "{}", program).expect("Unable to write to temp file");

    let result = interpret(
        file.path()
            .to_str()
            .expect("Unable to convert temp file path to string"),
    );
    assert_eq!(
        result,
        Err("Label with name: .newLabel doesn't exist".to_string())
    );
}

#[test]
pub fn test_bad_variable_name() {
    let mut file = NamedTempFile::new().expect("Unable to create temp file");
    let program = "LOAD_VAL 10\nWRITE_VAR 0x";
    write!(file, "{}", program).expect("Unable to write to temp file");

    let result = interpret(
        file.path()
            .to_str()
            .expect("Unable to convert temp file path to string"),
    );
    assert_eq!(
        result,
        Err("Unable to parse line #2: Invalid variable name 0x".to_string())
    );
}

#[test]
pub fn test_program_without_return() {
    let mut file = NamedTempFile::new().expect("Unable to create temp file");
    let program =
        "LOAD_VAL 10
        WRITE_VAR x
        LOAD_VAL 20
        WRITE_VAR y
        READ_VAR x
        READ_VAR y
        LOAD_VAL 10
        MULTIPLY
        ADD";
    write!(file, "{}", program).expect("Unable to write to temp file");

    let result = interpret(
        file.path()
            .to_str()
            .expect("Unable to convert temp file path to string"),
    );
    assert!(result.is_ok());
    assert!(result.as_ref().ok().unwrap().is_none());
}

#[test]
pub fn test_simple_program() {
    let mut file = NamedTempFile::new().expect("Unable to create temp file");
    let program =
        "LOAD_VAL 10
        WRITE_VAR x
        LOAD_VAL 20
        WRITE_VAR y
        READ_VAR x
        READ_VAR y
        LOAD_VAL 10
        MULTIPLY
        ADD
        RETURN_VALUE";
    write!(file, "{}", program).expect("Unable to write to temp file");

    let result = interpret(
        file.path()
            .to_str()
            .expect("Unable to convert temp file path to string"),
    );
    assert!(result.is_ok());
    assert!(result.as_ref().ok().is_some());
    assert_eq!(result.ok().unwrap().unwrap() as u16, 210);
}

#[test]
pub fn test_program_with_nested_loop() {
    let mut file = NamedTempFile::new().expect("Unable to create temp file");
    let program =
        "LOAD_VAL 0
        WRITE_VAR x
        LOAD_VAL 0
        WRITE_VAR y
        LOAD_VAL 0
        .first
        LOAD_VAL 1
        READ_VAR x
        ADD
        WRITE_VAR x
        LOAD_VAL 0
        .second
        LOAD_VAL 1
        READ_VAR y
        ADD
        WRITE_VAR y
        LOAD_VAL 1
        ADD
        DUP
        LOAD_VAL 10
        GREATER
        GOTO .second
        POP
        LOAD_VAL 1
        ADD
        DUP
        LOAD_VAL 10
        GREATER
        GOTO .first
        READ_VAR x
        READ_VAR y
        ADD
        RETURN_VALUE";
    write!(file, "{}", program).expect("Unable to write to temp file");

    let result = interpret(
        file.path()
            .to_str()
            .expect("Unable to convert temp file path to string"),
    );
    assert!(result.is_ok());
    assert!(result.as_ref().ok().is_some());
    assert_eq!(result.ok().unwrap().unwrap() as u16, 110);
}