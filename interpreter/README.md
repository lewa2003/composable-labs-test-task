# ByteCode interpreter

An interpreter for stack-based language ByteCode.

## Run

- Execute test file with ByteCode: `cargo run -- -f <test file>`

You can run tests by executing `cargo test`.

## Instructions

- `LOAD_VAL <value>`: pushes `<value>` to the stack;
- `WRITE_VAR <var name>`: pops value from the stack and saves it to the variable `<var name>`;
- `READ_VAR <var name>`: pushes the variable `<var name>` value to the stack;
- `ADD`: pops two values from the stack and pushes their sum;
- `MULTIPLY`: pops two values from the stack and pushes their product;
- `GREATER`: pops two values from the stack, pushes `1` if the first popped value is greater than the second, `0` otherwise;
- `LESS`: pops two values from the stack, pushes `1` if the first popped value is less than the second, `0` otherwise;
- `EQUAL`: pops two values from the stack, pushes `1` if values are equal and `0` otherwise;
- `DUP`: pops value from the stack and pushes two same values (duplicates the last value on the stack);
- `POP`: pops value from the stack;
- `.<label name>`: declares a label `<label name>`;
- `GOTO .<label name>`: pops value from the stack, if the popped value is `1` - moves the instruction pointer to the label `<label name>`;
- `RETURN_VALUE`: pops value from the stack and exits the program returning the popped value.

## Resources

Two ByteCode programs for tests

### simple_program.txt

```
function f() {
    x = 10
    y = 20
    return x + y * 10
}

result = 210
```

### program_with_nested_loops.txt

```
function f() {
    x = 0
    y = 0
    z = 0 
    for i = 0 to 10:
        x += 1
        for j = 0 to 10:
            y += 1
            for k = 0 to 10:
                z += 1
    return x + y + z
}

result = 1110
```

# Task 3: SEND_CHANNEL, RECV_CHANNEL and SPAWN

For `SEND_CHANNEL` and `RECV_CHANNEL` I will add `channel_stack: Stack<Channel>` to `struct Interpreter`.
To interpret this instruction I would use `std::sync::mpsc::Sender` and  `std::sync::mpsc::Receiver`.

For `SPAWN` I think it would be necessary to change an architecture a bit:
- add `Function` class witch will have its own `instructions: Vec::new()`
- add another label type to mark the beginning and the ending of functions
- add `function_stack: Stack<Function>` to `struct ByteCode`
- instancing `InterpreterState` for each function spawn

To interpret this instruction I would use `thread::spawn`.
