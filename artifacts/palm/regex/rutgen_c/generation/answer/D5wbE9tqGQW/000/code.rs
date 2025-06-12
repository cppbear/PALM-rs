// Answer 0

#[test]
fn test_needs_dotstar_dfa_enabled() {
    let program = Program {
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        ..Program::new()
    };
    assert_eq!(program.needs_dotstar(), true);
}

#[test]
fn test_needs_dotstar_dfa_enabled_reverse() {
    let program = Program {
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        ..Program::new()
    };
    assert_eq!(program.needs_dotstar(), false);
}

#[test]
fn test_needs_dotstar_dfa_enabled_anchored_start() {
    let program = Program {
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: true,
        ..Program::new()
    };
    assert_eq!(program.needs_dotstar(), false);
}

#[test]
fn test_needs_dotstar_dfa_disabled() {
    let program = Program {
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        ..Program::new()
    };
    assert_eq!(program.needs_dotstar(), false);
}

