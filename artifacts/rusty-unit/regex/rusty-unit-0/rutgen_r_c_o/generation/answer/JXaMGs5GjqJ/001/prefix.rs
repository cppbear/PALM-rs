// Answer 0

#[test]
fn test_shortest_nfa_start_at_0() {
    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };
    let text: &[u8] = &[0, 1, 2, 3, 4];
    exec.shortest_nfa(text, 0);
}

#[test]
fn test_shortest_nfa_start_at_1() {
    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec!["example".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };
    let text: &[u8] = &[0, 1, 2, 3, 4];
    exec.shortest_nfa(text, 1);
}

#[test]
fn test_shortest_nfa_start_at_2() {
    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec!["pattern".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };
    let text: &[u8] = &[0, 1, 2, 3, 4];
    exec.shortest_nfa(text, 2);
}

#[test]
fn test_shortest_nfa_start_at_3() {
    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec!["sample".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };
    let text: &[u8] = &[0, 1, 2, 3, 4];
    exec.shortest_nfa(text, 3);
}

#[test]
fn test_shortest_nfa_start_at_4() {
    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec!["string".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };
    let text: &[u8] = &[0, 1, 2, 3, 4];
    exec.shortest_nfa(text, 4);
}

#[test]
fn test_shortest_nfa_start_at_5() {
    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec!["final".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };
    let text: &[u8] = &[0, 1, 2, 3, 4];
    exec.shortest_nfa(text, 5);
}

