// Answer 0

#[test]
#[should_panic]
fn test_c_literal_empty_input() {
    let mut compiler = Compiler::new();
    let chars: Vec<char> = vec![];
    compiler.c_literal(&chars);
}

