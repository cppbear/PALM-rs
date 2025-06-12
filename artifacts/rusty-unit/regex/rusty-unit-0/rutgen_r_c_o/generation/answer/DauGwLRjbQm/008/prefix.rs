// Answer 0

#[test]
fn test_c_literal_with_valid_chars() {
    let mut compiler = Compiler::new().bytes(false).reverse(false);
    let chars = vec!['a', 'b', 'c', 'd', 'e'];
    compiler.c_literal(&chars).unwrap();
}

#[test]
fn test_c_literal_with_reversed_chars() {
    let mut compiler = Compiler::new().bytes(false).reverse(false);
    let chars = vec!['x', 'y', 'z'];
    compiler.c_literal(&chars).unwrap();
}

#[test]
fn test_c_literal_with_repeated_chars() {
    let mut compiler = Compiler::new().bytes(false).reverse(false);
    let chars = vec!['1', '1', '2', '3'];
    compiler.c_literal(&chars).unwrap();
}

#[test]
fn test_c_literal_with_unicode_chars() {
    let mut compiler = Compiler::new().bytes(false).reverse(false);
    let chars = vec!['α', 'β', 'γ', 'δ'];
    compiler.c_literal(&chars).unwrap();
}

#[test]
#[should_panic]
fn test_c_literal_with_empty_chars() {
    let mut compiler = Compiler::new().bytes(false).reverse(false);
    let chars: Vec<char> = vec![];
    compiler.c_literal(&chars);
}

