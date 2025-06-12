// Answer 0

#[test]
fn test_find_at_no_match_case() {
    let text: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let start: usize = 0;

    let exec_read_only = ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaAnchoredReverse,
    };
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &cache,
    };

    exec.find_at(text, start);
}

#[test]
fn test_find_at_no_match_case_with_start_zero() {
    let text: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let start: usize = 0;

    let exec_read_only = ExecReadOnly {
        res: vec!["example".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaAnchoredReverse,
    };
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &cache,
    };

    exec.find_at(text, start);
}

#[test]
fn test_find_at_no_match_case_with_varied_start() {
    let text: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    for start in 0..10 {
        let exec_read_only = ExecReadOnly {
            res: vec!["sample".to_string()],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::DfaAnchoredReverse,
        };
        let cache = RefCell::new(ProgramCacheInner::default());
        let exec = ExecNoSync {
            ro: &Arc::new(exec_read_only),
            cache: &cache,
        };

        exec.find_at(text, start);
    }
}

#[test]
#[should_panic]
fn test_find_at_should_panic_on_unreachable() {
    let text: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let start: usize = 5;

    let exec_read_only = ExecReadOnly {
        res: vec!["panic".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaMany,
    };
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &cache,
    };

    exec.find_at(text, start);
}

