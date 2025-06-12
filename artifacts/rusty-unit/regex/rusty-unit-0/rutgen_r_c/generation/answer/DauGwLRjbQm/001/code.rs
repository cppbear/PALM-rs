// Answer 0

#[test]
#[should_panic(expected = "non-empty literal")]
fn test_c_literal_empty_chars() {
    let mut compiler = Compiler::new();
    let chars: Vec<char> = vec![]; // empty vector to trigger the assertion
    compiler.c_literal(&chars).unwrap();
}

#[test]
fn test_c_literal_single_char() {
    let mut compiler = Compiler::new();
    let chars: Vec<char> = vec!['a'];
    let result = compiler.c_literal(&chars).unwrap();
    assert!(result.hole != Hole::None); // Check that the hole is filled
}

#[test]
fn test_c_literal_multiple_chars() {
    let mut compiler = Compiler::new();
    let chars: Vec<char> = vec!['a', 'b', 'c'];
    let result = compiler.c_literal(&chars).unwrap();
    assert!(result.hole != Hole::None); // Check that the hole is filled
}

#[test]
fn test_c_literal_reverse_order() {
    let mut compiler = Compiler::new().reverse(true);
    let chars: Vec<char> = vec!['a', 'b', 'c'];
    let result = compiler.c_literal(&chars).unwrap();
    assert!(result.hole != Hole::None); // Check that the hole is filled
}

