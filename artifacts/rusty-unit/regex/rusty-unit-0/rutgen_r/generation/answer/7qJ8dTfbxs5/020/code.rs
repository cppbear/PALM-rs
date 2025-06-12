// Answer 0

#[test]
fn test_follow_epsilons_start_line() {
    struct TestDFA {
        cache: TestCache,
        prog: Vec<prog::Inst>,
    }

    struct TestCache {
        stack: Vec<InstPtr>,
    }

    struct EmptyFlags {
        start_line: bool,
        end_line: bool,
        start: bool,
        end: bool,
        word_boundary: bool,
        not_word_boundary: bool,
    }

    let mut dfa = TestDFA {
        cache: TestCache { stack: vec![0] },
        prog: vec![
            prog::Inst::EmptyLook(prog::EmptyLook { look: prog::EmptyLookKind::StartLine, goto: 1 }),
            prog::Inst::Match(prog::Match {}),
            prog::Inst::EmptyLook(prog::EmptyLook { look: prog::EmptyLookKind::EndLine, goto: 2 }),
            prog::Inst::Match(prog::Match {}),
        ],
    };

    let mut q = SparseSet::new();
    let flags = EmptyFlags {
        start_line: true,
        end_line: false,
        start: false,
        end: false,
        word_boundary: false,
        not_word_boundary: false,
    };

    dfa.follow_epsilons(0, &mut q, flags);

    assert!(q.contains(0));
    assert!(q.contains(1));
}

#[test]
fn test_follow_epsilons_end_line() {
    struct TestDFA {
        cache: TestCache,
        prog: Vec<prog::Inst>,
    }

    struct TestCache {
        stack: Vec<InstPtr>,
    }

    struct EmptyFlags {
        start_line: bool,
        end_line: bool,
        start: bool,
        end: bool,
        word_boundary: bool,
        not_word_boundary: bool,
    }

    let mut dfa = TestDFA {
        cache: TestCache { stack: vec![0] },
        prog: vec![
            prog::Inst::EmptyLook(prog::EmptyLook { look: prog::EmptyLookKind::EndLine, goto: 1 }),
            prog::Inst::Match(prog::Match {}),
            prog::Inst::EmptyLook(prog::EmptyLook { look: prog::EmptyLookKind::StartLine, goto: 2 }),
            prog::Inst::Match(prog::Match {}),
        ],
    };

    let mut q = SparseSet::new();
    let flags = EmptyFlags {
        start_line: false,
        end_line: true,
        start: false,
        end: false,
        word_boundary: false,
        not_word_boundary: false,
    };

    dfa.follow_epsilons(0, &mut q, flags);

    assert!(q.contains(0));
    assert!(q.contains(1));
}

