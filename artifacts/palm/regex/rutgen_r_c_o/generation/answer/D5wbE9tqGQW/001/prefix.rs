// Answer 0

#[test]
fn test_needs_dotstar_case_1() {
    let program = Program {
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        ..Program::new()
    };
    let _ = program.needs_dotstar();
}

#[test]
fn test_needs_dotstar_case_2() {
    let program = Program {
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: true,
        ..Program::new()
    };
    let _ = program.needs_dotstar();
}

#[test]
fn test_needs_dotstar_case_3() {
    let program = Program {
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        ..Program::new()
    };
    let _ = program.needs_dotstar();
}

