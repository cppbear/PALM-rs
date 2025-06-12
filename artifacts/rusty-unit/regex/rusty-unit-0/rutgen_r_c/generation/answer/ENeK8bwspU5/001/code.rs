// Answer 0

#[test]
fn test_bytes_true() {
    let compiler = Compiler::new();
    let modified_compiler = compiler.bytes(true);
    assert!(modified_compiler.compiled.is_bytes);
    assert!(!modified_compiler.compiled.only_utf8);
}

#[test]
fn test_bytes_false() {
    let compiler = Compiler::new();
    let modified_compiler = compiler.bytes(false);
    assert!(!modified_compiler.compiled.is_bytes);
    assert!(!modified_compiler.compiled.only_utf8);
}

#[test]
fn test_bytes_true_with_dfa() {
    let compiler = Compiler::new();
    let modified_compiler = compiler.dfa(true).bytes(true);
    assert!(modified_compiler.compiled.is_bytes);
    assert!(modified_compiler.compiled.is_dfa);
}

#[test]
fn test_bytes_false_with_dfa() {
    let compiler = Compiler::new();
    let modified_compiler = compiler.dfa(true).bytes(false);
    assert!(!modified_compiler.compiled.is_bytes);
    assert!(modified_compiler.compiled.is_dfa);
}

#[test]
fn test_bytes_interaction_with_utf8() {
    let compiler = Compiler::new().bytes(false).only_utf8(true);
    assert!(!compiler.compiled.is_bytes);
    assert!(compiler.compiled.only_utf8);
}

#[test]
fn test_bytes_and_utf8_coexistence() {
    let compiler = Compiler::new().bytes(true).only_utf8(false);
    assert!(compiler.compiled.is_bytes);
    assert!(!compiler.compiled.only_utf8);
}

