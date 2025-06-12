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
fn test_can_exec_with_exceeding_instruction_length() {
    struct Program {
        dfa_size_limit: usize,
        instructions: Vec<prog::Inst>,
    }

    let program = Program {
        dfa_size_limit: 1,
        instructions: vec![prog::Inst::Match; std::i32::MAX as usize],
    };

    assert_eq!(can_exec(&program), false);
}

#[test]
fn test_can_exec_with_match_instruction() {
    struct Program {
        dfa_size_limit: usize,
        instructions: Vec<prog::Inst>,
    }

    let program = Program {
        dfa_size_limit: 1,
        instructions: vec![prog::Inst::Match],
    };

    assert_eq!(can_exec(&program), false);
}

#[test]
fn test_can_exec_with_valid_program() {
    struct Program {
        dfa_size_limit: usize,
        instructions: Vec<prog::Inst>,
    }

    let program = Program {
        dfa_size_limit: 1,
        instructions: vec![
            prog::Inst::EmptyLook,
            prog::Inst::Save,
            prog::Inst::Split,
            prog::Inst::Bytes,
            prog::Inst::Match,
        ],
    };

    assert_eq!(can_exec(&program), false);
}

