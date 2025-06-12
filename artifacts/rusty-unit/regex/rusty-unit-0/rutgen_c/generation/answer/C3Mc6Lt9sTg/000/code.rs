// Answer 0

#[test]
fn test_reverse_true() {
    let compiler = Compiler::new().reverse(true);
    assert!(compiler.compiled.is_reverse);
}

#[test]
fn test_reverse_false() {
    let compiler = Compiler::new().reverse(false);
    assert!(!compiler.compiled.is_reverse);
}

