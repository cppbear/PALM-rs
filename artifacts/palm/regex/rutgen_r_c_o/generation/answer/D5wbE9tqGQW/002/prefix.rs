// Answer 0

#[test]
fn test_needs_dotstar_case1() {
    let program = Program {
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        ..Program::new()
    };
    program.needs_dotstar();
}

#[test]
fn test_needs_dotstar_case2() {
    let program = Program {
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        ..Program::new()
    };
    program.needs_dotstar();
}

#[test]
fn test_needs_dotstar_case3() {
    let program = Program {
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        ..Program::new()
    };
    program.needs_dotstar();
}

