// Answer 0

#[test]
fn test_uses_bytes_is_bytes_true() {
    let program = Program {
        is_bytes: true,
        is_dfa: false,
        ..Program::new()  // initialize other fields with default values
    };
    assert_eq!(program.uses_bytes(), true);
}

#[test]
fn test_uses_bytes_is_bytes_false_is_dfa_true() {
    let program = Program {
        is_bytes: false,
        is_dfa: true,
        ..Program::new()  // initialize other fields with default values
    };
    assert_eq!(program.uses_bytes(), true);
}

#[test]
fn test_uses_bytes_both_false() {
    let program = Program {
        is_bytes: false,
        is_dfa: false,
        ..Program::new()  // initialize other fields with default values
    };
    assert_eq!(program.uses_bytes(), false);
}

#[test]
fn test_uses_bytes_both_true() {
    let program = Program {
        is_bytes: true,
        is_dfa: true,
        ..Program::new()  // initialize other fields with default values
    };
    assert_eq!(program.uses_bytes(), true);
}

