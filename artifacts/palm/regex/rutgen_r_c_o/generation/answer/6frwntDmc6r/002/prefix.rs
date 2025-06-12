// Answer 0

#[test]
fn test_shortest_nfa_type_auto_false() {
    let ty = MatchNfaType::Auto;
    let text: &[u8] = b"abc";
    let start = 0;

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

    let result = exec_no_sync.shortest_nfa_type(ty, text, start);
}

#[test]
fn test_shortest_nfa_type_auto_false_start_one() {
    let ty = MatchNfaType::Auto;
    let text: &[u8] = b"abc";
    let start = 1;

    let exec_read_only = ExecReadOnly {
        res: vec!["b".to_string()],
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

    let result = exec_no_sync.shortest_nfa_type(ty, text, start);
}

#[test]
fn test_shortest_nfa_type_auto_empty_text() {
    let ty = MatchNfaType::Auto;
    let text: &[u8] = b"";
    let start = 0;

    let exec_read_only = ExecReadOnly {
        res: vec!["".to_string()],
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

    let result = exec_no_sync.shortest_nfa_type(ty, text, start);
}

