// Answer 0

#[test]
fn test_bytes_enable() {
    let compiler = Compiler::new();
    let modified_compiler = compiler.bytes(true);
    assert!(modified_compiler.compiled.is_bytes);
}

#[test]
fn test_bytes_disable() {
    let compiler = Compiler::new();
    let modified_compiler = compiler.bytes(false);
    assert!(!modified_compiler.compiled.is_bytes);
}

#[test]
fn test_dfa_implies_bytes() {
    let compiler = Compiler::new();
    let modified_compiler = compiler.dfa(true);
    assert!(modified_compiler.compiled.is_bytes);
}

