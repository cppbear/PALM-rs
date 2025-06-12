// Answer 0

#[test]
fn test_match_nfa_empty_text() {
    let exec_read_only = ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    };
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &cache,
    };

    let text: &[u8] = b"";
    assert!(!exec_no_sync.match_nfa(text, 0));
}

#[test]
fn test_match_nfa_starting_with_match() {
    let exec_read_only = ExecReadOnly {
        res: vec!["ab".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    };
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &cache,
    };

    let text: &[u8] = b"abc";
    assert!(exec_no_sync.match_nfa(text, 0));
}

#[test]
fn test_match_nfa_no_match() {
    let exec_read_only = ExecReadOnly {
        res: vec!["xyz".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    };
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &cache,
    };

    let text: &[u8] = b"abc";
    assert!(!exec_no_sync.match_nfa(text, 0));
}

#[test]
fn test_match_nfa_partial_match() {
    let exec_read_only = ExecReadOnly {
        res: vec!["abc".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    };
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &cache,
    };

    let text: &[u8] = b"ab";
    assert!(!exec_no_sync.match_nfa(text, 0));
}

#[test]
fn test_match_nfa_edge_case() {
    let exec_read_only = ExecReadOnly {
        res: vec!["a".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    };
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &cache,
    };

    let text: &[u8] = b"a";
    assert!(exec_no_sync.match_nfa(text, 0));
    
    let text: &[u8] = b"";
    assert!(!exec_no_sync.match_nfa(text, 1));
}

