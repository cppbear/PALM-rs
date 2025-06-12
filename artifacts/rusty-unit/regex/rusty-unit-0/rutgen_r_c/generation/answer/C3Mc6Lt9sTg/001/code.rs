// Answer 0

#[test]
fn test_reverse_functionality_true() {
    let compiler = Compiler::new();
    let modified_compiler = compiler.reverse(true);
    assert!(modified_compiler.compiled.is_reverse);
}

#[test]
fn test_reverse_functionality_false() {
    let compiler = Compiler::new();
    let modified_compiler = compiler.reverse(false);
    assert!(!modified_compiler.compiled.is_reverse);
}

#[test]
fn test_reverse_functional_chaining() {
    let compiler = Compiler::new();
    let modified_compiler = compiler.reverse(true).reverse(false);
    assert!(!modified_compiler.compiled.is_reverse);
}

