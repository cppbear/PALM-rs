// Answer 0

#[derive(Debug)]
struct Program {
    is_bytes: bool,
    is_dfa: bool,
}

impl Program {
    pub fn uses_bytes(&self) -> bool {
        self.is_bytes || self.is_dfa
    }
}

#[test]
fn test_uses_bytes_when_is_bytes_true() {
    let program = Program {
        is_bytes: true,
        is_dfa: false,
    };
    assert_eq!(program.uses_bytes(), true);
}

#[test]
fn test_uses_bytes_when_is_dfa_true() {
    let program = Program {
        is_bytes: false,
        is_dfa: true,
    };
    assert_eq!(program.uses_bytes(), true);
}

#[test]
fn test_uses_bytes_when_both_false() {
    let program = Program {
        is_bytes: false,
        is_dfa: false,
    };
    assert_eq!(program.uses_bytes(), false);
}

#[test]
fn test_uses_bytes_when_both_true() {
    let program = Program {
        is_bytes: true,
        is_dfa: true,
    };
    assert_eq!(program.uses_bytes(), true);
}

