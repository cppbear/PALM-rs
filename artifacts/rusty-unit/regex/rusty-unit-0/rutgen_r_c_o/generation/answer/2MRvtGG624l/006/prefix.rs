// Answer 0

#[test]
fn test_find_at_match_type_dfa_suffix_case_1() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let text: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let start: usize = 0;
    exec.find_at(text, start);
}

#[test]
fn test_find_at_match_type_dfa_suffix_case_2() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["example".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let text: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let start: usize = 5;
    exec.find_at(text, start);
}

#[test]
fn test_find_at_match_type_dfa_suffix_case_3() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["testing".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let text: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let start: usize = 9;
    exec.find_at(text, start);
}

#[test]
fn test_find_at_match_type_dfa_suffix_case_4() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["sample".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let text: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let start: usize = 1;
    exec.find_at(text, start);
}

#[test]
fn test_find_at_match_type_dfa_suffix_case_5() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["rust".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let text: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let start: usize = 3;
    exec.find_at(text, start);
}

