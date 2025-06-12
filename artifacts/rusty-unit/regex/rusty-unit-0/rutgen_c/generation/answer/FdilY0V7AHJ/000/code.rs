// Answer 0

#[test]
fn test_only_utf8_enabled() {
    let compiler = Compiler::new();
    let modified_compiler = compiler.only_utf8(true);
    assert!(modified_compiler.compiled.only_utf8);
}

#[test]
fn test_only_utf8_disabled() {
    let compiler = Compiler::new();
    let modified_compiler = compiler.only_utf8(false);
    assert!(!modified_compiler.compiled.only_utf8);
}

