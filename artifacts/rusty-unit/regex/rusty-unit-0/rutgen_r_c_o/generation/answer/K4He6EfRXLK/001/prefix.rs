// Answer 0

#[test]
fn test_shortest_dfa_reverse_suffix_case1() {
    let text: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let start: usize = 0;

    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("test")],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache: ProgramCache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    exec.shortest_dfa_reverse_suffix(text, start);
}

#[test]
fn test_shortest_dfa_reverse_suffix_case2() {
    let text: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let start: usize = 9;

    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("test")],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache: ProgramCache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    exec.shortest_dfa_reverse_suffix(text, start);
}

#[test]
fn test_shortest_dfa_reverse_suffix_case3() {
    let text: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
    let start: usize = 0;

    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("example")],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache: ProgramCache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    exec.shortest_dfa_reverse_suffix(text, start);
}

#[test]
fn test_shortest_dfa_reverse_suffix_case4() {
    let text: &[u8] = &[0, 2, 3, 4, 2, 5, 6, 7, 8, 9];
    let start: usize = 5;

    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("testing")],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache: ProgramCache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    exec.shortest_dfa_reverse_suffix(text, start);
}

