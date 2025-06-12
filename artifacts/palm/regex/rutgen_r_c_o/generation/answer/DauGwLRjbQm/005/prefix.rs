// Answer 0

#[test]
fn test_c_literal_single_char_reverse() {
    let mut compiler = Compiler::new();
    compiler.compiled.is_reverse = true;

    let chars = vec!['a'];
    let _ = compiler.c_literal(&chars);
}

#[test]
fn test_c_literal_multiple_chars_reverse() {
    let mut compiler = Compiler::new();
    compiler.compiled.is_reverse = true;

    let chars = vec!['a', 'b', 'c'];
    let _ = compiler.c_literal(&chars);
}

#[test]
fn test_c_literal_with_unicode_reverse() {
    let mut compiler = Compiler::new();
    compiler.compiled.is_reverse = true;

    let chars = vec!['😀', '😁', '😂'];
    let _ = compiler.c_literal(&chars);
}

#[test]
fn test_c_literal_with_longer_sequence_reverse() {
    let mut compiler = Compiler::new();
    compiler.compiled.is_reverse = true;

    let chars: Vec<char> = ('a'..='z').collect();
    let _ = compiler.c_literal(&chars);
}

