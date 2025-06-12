// Answer 0

#[test]
fn test_slots_len_with_no_captures() {
    struct DummyProgram {
        captures: Vec<usize>,
    }

    let program = DummyProgram { captures: vec![] };    
    let exec_read_only = ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: program,
        dfa: program,
        dfa_reverse: program,
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    };
    let exec = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    assert_eq!(exec.slots_len(), 0);
}

#[test]
fn test_slots_len_with_one_capture() {
    struct DummyProgram {
        captures: Vec<usize>,
    }

    let program = DummyProgram { captures: vec![0] };    
    let exec_read_only = ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: program,
        dfa: program,
        dfa_reverse: program,
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    };
    let exec = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    assert_eq!(exec.slots_len(), 2);
}

#[test]
fn test_slots_len_with_multiple_captures() {
    struct DummyProgram {
        captures: Vec<usize>,
    }

    let program = DummyProgram { captures: vec![0, 1, 2] };    
    let exec_read_only = ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: program,
        dfa: program,
        dfa_reverse: program,
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    };
    let exec = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    assert_eq!(exec.slots_len(), 6);
}

