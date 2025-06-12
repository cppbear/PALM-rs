// Answer 0

#[test]
fn test_find_at_case_1() {
    let text: &[u8] = b"abcd";
    let start = 0;
    let match_type = MatchType::Dfa;
    let ro = ExecReadOnly {
        res: vec!["abc".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type,
    };
    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec.find_at(text, start);
}

#[test]
fn test_find_at_case_2() {
    let text: &[u8] = b"abababab";
    let start = 4;
    let match_type = MatchType::Dfa;
    let ro = ExecReadOnly {
        res: vec!["ab".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type,
    };
    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec.find_at(text, start);
}

#[test]
fn test_find_at_case_3() {
    let text: &[u8] = b"xyzxyz";
    let start = 0;
    let match_type = MatchType::Dfa;
    let ro = ExecReadOnly {
        res: vec!["xyz".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type,
    };
    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec.find_at(text, start);
}

#[test]
fn test_find_at_case_4() {
    let text: &[u8] = b"aaaaaaaaaaaaaa";
    let start = 0;
    let match_type = MatchType::Dfa;
    let ro = ExecReadOnly {
        res: vec!["a".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type,
    };
    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec.find_at(text, start);
}

#[test]
fn test_find_at_case_5() {
    let text: &[u8] = b"longtextstring";
    let start = 5;
    let match_type = MatchType::Dfa;
    let ro = ExecReadOnly {
        res: vec!["text".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type,
    };
    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec.find_at(text, start);
}

