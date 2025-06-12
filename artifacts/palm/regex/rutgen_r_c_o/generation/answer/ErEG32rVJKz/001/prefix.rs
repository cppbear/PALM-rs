// Answer 0

#[test]
fn test_exec_backtrack_empty_text() {
    let matches = &mut [false; 256];
    let slots = &mut [];
    let text: &[u8] = &[];
    let start = 0;

    let program = Program::new();
    let exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec!["a".to_string()],
            nfa: program,
            dfa: program.clone(),
            dfa_reverse: program.clone(),
            suffixes: LiteralSearcher::empty(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    exec.exec_backtrack(matches, slots, text, start);
}

#[test]
fn test_exec_backtrack_short_text() {
    let matches = &mut [false; 256];
    let slots = &mut [];
    let text: &[u8] = &[b'a', b'b', b'c'];
    let start = 0;

    let program = Program::new();
    let exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec!["abc".to_string()],
            nfa: program,
            dfa: program.clone(),
            dfa_reverse: program.clone(),
            suffixes: LiteralSearcher::empty(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    exec.exec_backtrack(matches, slots, text, start);
}

#[test]
fn test_exec_backtrack_long_text() {
    let matches = &mut [false; 256];
    let slots = &mut [];
    let text: &[u8] = &[0; 10_000]; // long text filled with zeros
    let start = 0;

    let program = Program::new();
    let exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec!["a".to_string()],
            nfa: program,
            dfa: program.clone(),
            dfa_reverse: program.clone(),
            suffixes: LiteralSearcher::empty(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    exec.exec_backtrack(matches, slots, text, start);
}

#[test]
fn test_exec_backtrack_with_panic() {
    let matches = &mut [false; 256];
    let slots = &mut [];
    let text: &[u8] = &[b'x', b'y', b'z']; // some input
    let start = 9999; // out of bounds

    let program = Program::new();
    let exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec!["xyz".to_string()],
            nfa: program,
            dfa: program.clone(),
            dfa_reverse: program.clone(),
            suffixes: LiteralSearcher::empty(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    let result = std::panic::catch_unwind(|| {
        exec.exec_backtrack(matches, slots, text, start);
    });
    
    assert!(result.is_err());
}

#[test]
fn test_exec_backtrack_max_start() {
    let matches = &mut [false; 256];
    let slots = &mut [];
    let text: &[u8] = &[b'a', b'b', b'c'];
    let start = 10_000; // maximum out of bounds start

    let program = Program::new();
    let exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec!["abc".to_string()],
            nfa: program,
            dfa: program.clone(),
            dfa_reverse: program.clone(),
            suffixes: LiteralSearcher::empty(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    let result = std::panic::catch_unwind(|| {
        exec.exec_backtrack(matches, slots, text, start);
    });
    
    assert!(result.is_err());
}

