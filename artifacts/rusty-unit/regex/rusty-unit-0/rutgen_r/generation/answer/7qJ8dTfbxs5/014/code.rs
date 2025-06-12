// Answer 0

#[test]
fn test_follow_epsilons_with_valid_path() {
    struct TestDFA {
        cache: TestCache,
        prog: Vec<prog::Inst>,
    }

    struct TestCache {
        stack: Vec<InstPtr>,
    }

    let mut dfa = TestDFA {
        cache: TestCache { stack: vec![0] },
        prog: vec![
            prog::Inst::EmptyLook(prog::EmptyLook { look: prog::EmptyLookType::EndText, goto: 1 }),
            prog::Inst::EmptyLook(prog::EmptyLook { look: prog::EmptyLookType::StartLine, goto: 2 }),
            prog::Inst::Match(prog::Match),
        ],
    };

    let mut q = SparseSet::new();
    let flags = EmptyFlags { start_line: false, end_line: true, start: false, end: true, word_boundary: false, not_word_boundary: false };

    dfa.follow_epsilons(0, &mut q, flags);

    assert!(q.contains(0));
    assert!(q.contains(1));
}

#[test]
fn test_follow_epsilons_with_epsilon_transition() {
    struct TestDFA {
        cache: TestCache,
        prog: Vec<prog::Inst>,
    }

    struct TestCache {
        stack: Vec<InstPtr>,
    }

    let mut dfa = TestDFA {
        cache: TestCache { stack: vec![0] },
        prog: vec![
            prog::Inst::EmptyLook(prog::EmptyLook { look: prog::EmptyLookType::EndLine, goto: 1 }),
            prog::Inst::EmptyLook(prog::EmptyLook { look: prog::EmptyLookType::WordBoundary, goto: 2 }),
            prog::Inst::Match(prog::Match),
        ],
    };

    let mut q = SparseSet::new();
    let flags = EmptyFlags { start_line: false, end_line: true, start: false, end: false, word_boundary: true, not_word_boundary: false };

    dfa.follow_epsilons(0, &mut q, flags);

    assert!(q.contains(0));
    assert!(q.contains(1));
    assert!(q.contains(2));
}

#[test]
#[should_panic]
fn test_follow_epsilons_with_already_visited_state() {
    struct TestDFA {
        cache: TestCache,
        prog: Vec<prog::Inst>,
    }

    struct TestCache {
        stack: Vec<InstPtr>,
    }

    let mut dfa = TestDFA {
        cache: TestCache { stack: vec![0] },
        prog: vec![
            prog::Inst::EmptyLook(prog::EmptyLook { look: prog::EmptyLookType::EndText, goto: 1 }),
            prog::Inst::EmptyLook(prog::EmptyLook { look: prog::EmptyLookType::StartLine, goto: 0 }), // Points back to itself.
            prog::Inst::Match(prog::Match),
        ],
    };

    let mut q = SparseSet::new();
    let flags = EmptyFlags { start_line: false, end_line: true, start: false, end: true, word_boundary: false, not_word_boundary: false };

    dfa.follow_epsilons(0, &mut q, flags);
}

