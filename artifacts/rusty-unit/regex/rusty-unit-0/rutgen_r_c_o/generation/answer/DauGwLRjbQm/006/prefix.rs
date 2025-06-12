// Answer 0

#[test]
fn test_c_literal_valid_input_err_first_c_char() {
    let mut compiler = Compiler::new();
    compiler.compiled.is_reverse = false;

    let chars: &[char] = &['a', 'b', 'c'];

    let _ = compiler.c_literal(chars);
}

#[test]
fn test_c_literal_valid_input_err_second_c_char() {
    let mut compiler = Compiler::new();
    compiler.compiled.is_reverse = false;

    let chars: &[char] = &['x', 'y', 'z'];

    let _ = compiler.c_literal(chars);
}

#[test]
fn test_c_literal_valid_input_sequence() {
    let mut compiler = Compiler::new();
    compiler.compiled.is_reverse = false;

    let chars: &[char] = &['1', '2', '3'];

    let _ = compiler.c_literal(chars);
}

