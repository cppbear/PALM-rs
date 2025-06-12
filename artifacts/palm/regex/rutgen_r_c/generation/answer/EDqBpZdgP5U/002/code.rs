// Answer 0

#[test]
fn test_uses_bytes_when_is_dfa_true() {
    let program = Program {
        is_bytes: false,
        is_dfa: true,
        ..Program::new()
    };
    assert_eq!(program.uses_bytes(), true);
}

#[test]
fn test_uses_bytes_when_is_dfa_false() {
    let program = Program {
        is_bytes: false,
        is_dfa: false,
        ..Program::new()
    };
    assert_eq!(program.uses_bytes(), false);
}

