// Answer 0

#[test]
fn test_needs_dotstar_case_1() {
    let program = Program {
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        ..Program::new()
    };
    assert!(!program.needs_dotstar());
}

#[test]
fn test_needs_dotstar_case_2() {
    let program = Program {
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        ..Program::new()
    };
    assert!(program.needs_dotstar());
}

#[test]
fn test_needs_dotstar_case_3() {
    let program = Program {
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: true,
        ..Program::new()
    };
    assert!(!program.needs_dotstar());
}

#[test]
fn test_needs_dotstar_case_4() {
    let program = Program {
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        ..Program::new()
    };
    assert!(!program.needs_dotstar());
}

#[test]
fn test_needs_dotstar_case_5() {
    let program = Program {
        is_dfa: false,
        is_reverse: true,
        is_anchored_start: false,
        ..Program::new()
    };
    assert!(!program.needs_dotstar());
}

