// Answer 0

#[test]
fn test_slots_len_empty_captures() {
    struct TestExecReadOnly {
        nfa: Program,
    }

    let nfa = Program { captures: vec![] };
    let ro = TestExecReadOnly { nfa };
    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    assert_eq!(exec.slots_len(), 0);
}

#[test]
fn test_slots_len_single_capture() {
    struct TestExecReadOnly {
        nfa: Program,
    }

    let nfa = Program { captures: vec![0] }; // One capture group
    let ro = TestExecReadOnly { nfa };
    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    assert_eq!(exec.slots_len(), 2); // Two slots for one capture
}

#[test]
fn test_slots_len_multiple_captures() {
    struct TestExecReadOnly {
        nfa: Program,
    }

    let nfa = Program { captures: vec![0, 1, 2] }; // Three capture groups
    let ro = TestExecReadOnly { nfa };
    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    assert_eq!(exec.slots_len(), 6); // Six slots for three captures
}

