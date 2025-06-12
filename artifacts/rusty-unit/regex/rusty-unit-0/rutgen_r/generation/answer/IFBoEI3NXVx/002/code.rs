// Answer 0

#[test]
fn test_can_exec_with_zero_dfa_size_limit() {
    struct Program<'a> {
        dfa_size_limit: usize,
        instructions: &'a [prog::Inst<'a>],
    }

    let instructions = &[prog::Inst::EmptyLook, prog::Inst::Match];
    let program = Program {
        dfa_size_limit: 0,
        instructions,
    };

    assert_eq!(can_exec(&program), false);
}

#[test]
fn test_can_exec_with_exceeding_length() {
    struct Program<'a> {
        dfa_size_limit: usize,
        instructions: &'a [prog::Inst<'a>],
    }

    let instructions = vec![prog::Inst::Save; (::std::i32::MAX as usize) + 1];
    let program = Program {
        dfa_size_limit: 1, // Arbitrarily non-zero to avoid early return
        instructions: &instructions,
    };

    assert_eq!(can_exec(&program), false);
}

