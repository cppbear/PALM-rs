// Answer 0

#[test]
fn test_dfa_set_true() {
    let compiler = Compiler::new();
    let updated_compiler = compiler.dfa(true);
    assert_eq!(updated_compiler.compiled.is_dfa, true);
}

#[test]
fn test_dfa_set_false() {
    let compiler = Compiler::new();
    let updated_compiler = compiler.dfa(false);
    assert_eq!(updated_compiler.compiled.is_dfa, false);
}

#[test]
fn test_dfa_chain_set() {
    let compiler = Compiler::new();
    let updated_compiler = compiler.dfa(true).dfa(false);
    assert_eq!(updated_compiler.compiled.is_dfa, false);
}

