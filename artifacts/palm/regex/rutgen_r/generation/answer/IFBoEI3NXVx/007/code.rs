// Answer 0

#[test]
fn test_can_exec_with_dfa_size_limit_zero() {
    struct Program {
        dfa_size_limit: usize,
        instructions: Vec<prog::Inst>,
    }

    let program = Program {
        dfa_size_limit: 0,
        instructions: vec![],
    };

    assert_eq!(can_exec(&program), false);
}

#[test]
fn test_can_exec_with_exceeding_length() {
    struct Program {
        dfa_size_limit: usize,
        instructions: Vec<prog::Inst>,
    }

    let program = Program {
        dfa_size_limit: 1,
        instructions: vec![prog::Inst::Split(0)],    
    };

    assert_eq!(can_exec(&program), true);
}

#[test]
fn test_can_exec_with_split_instruction() {
    struct Program {
        dfa_size_limit: usize,
        instructions: Vec<prog::Inst>,
    }

    let program = Program {
        dfa_size_limit: 1,
        instructions: vec![prog::Inst::Split(0)],
    };

    assert_eq!(can_exec(&program), true);
}

#[test]
fn test_can_exec_with_char_instruction() {
    struct Program {
        dfa_size_limit: usize,
        instructions: Vec<prog::Inst>,
    }

    let program = Program {
        dfa_size_limit: 1,
        instructions: vec![prog::Inst::Char(b'a')],
    };

    assert_eq!(can_exec(&program), false);
}

#[test]
fn test_can_exec_with_ranges_instruction() {
    struct Program {
        dfa_size_limit: usize,
        instructions: Vec<prog::Inst>,
    }

    let program = Program {
        dfa_size_limit: 1,
        instructions: vec![prog::Inst::Ranges(b'a'..=b'z')],
    };

    assert_eq!(can_exec(&program), false);
}

