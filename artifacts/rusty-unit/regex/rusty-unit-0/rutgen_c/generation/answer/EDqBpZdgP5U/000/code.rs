// Answer 0

#[test]
fn test_uses_bytes_when_is_bytes_is_true() {
    let program = Program {
        is_bytes: true,
        is_dfa: false,
        ..Program::new()
    };
    assert!(program.uses_bytes());
}

#[test]
fn test_uses_bytes_when_is_dfa_is_true() {
    let program = Program {
        is_bytes: false,
        is_dfa: true,
        ..Program::new()
    };
    assert!(program.uses_bytes());
}

#[test]
fn test_uses_bytes_when_both_are_false() {
    let program = Program {
        is_bytes: false,
        is_dfa: false,
        ..Program::new()
    };
    assert!(!program.uses_bytes());
}

#[test]
fn test_uses_bytes_when_both_are_true() {
    let program = Program {
        is_bytes: true,
        is_dfa: true,
        ..Program::new()
    };
    assert!(program.uses_bytes());
}

