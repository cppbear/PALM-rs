// Answer 0

#[test]
fn test_uses_bytes_with_is_dfa_true() {
    struct Program {
        is_bytes: bool,
        is_dfa: bool,
    }

    let program = Program {
        is_bytes: false,
        is_dfa: true,
    };

    assert_eq!(program.uses_bytes(), true);
}

#[test]
fn test_uses_bytes_with_is_dfa_false() {
    struct Program {
        is_bytes: bool,
        is_dfa: bool,
    }

    let program = Program {
        is_bytes: false,
        is_dfa: false,
    };

    assert_eq!(program.uses_bytes(), false);
}

