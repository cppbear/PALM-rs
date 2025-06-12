// Answer 0

#[test]
fn test_uses_bytes_false_dfa_false() {
    let program = Program {
        is_bytes: false,
        is_dfa: false,
        ..Program::new()
    };
    program.uses_bytes();
}

#[test]
fn test_uses_bytes_true_dfa_true() {
    let program = Program {
        is_bytes: true,
        is_dfa: true,
        ..Program::new()
    };
    program.uses_bytes();
}

#[test]
fn test_uses_bytes_false_dfa_true() {
    let program = Program {
        is_bytes: false,
        is_dfa: true,
        ..Program::new()
    };
    program.uses_bytes();
}

#[test]
fn test_uses_bytes_true_dfa_false() {
    let program = Program {
        is_bytes: true,
        is_dfa: false,
        ..Program::new()
    };
    program.uses_bytes();
}

