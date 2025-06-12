// Answer 0

#[test]
fn test_find_at_no_match_case() {
    let text: &[u8] = &[0, 1, 2, 3, 4, 5];
    let start = 0;

    let ro = Arc::new(ExecReadOnly {
        res: vec!["test_regex".to_string()],
        nfa: Program::default(),
        dfa: Program::default(), // Empty or non-matching DFA
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::new(vec![]),
        match_type: MatchType::Dfa,
    });

    let cache = RefCell::new(ProgramCacheInner::default());
    let executor = ExecNoSync { ro: &ro, cache: &cache };

    let result = executor.find_at(text, start);
}

#[test]
fn test_find_at_no_match_case_with_different_start() {
    let text: &[u8] = &[0, 1, 2, 3, 4, 5];
    let start = 4;

    let ro = Arc::new(ExecReadOnly {
        res: vec!["test_regex".to_string()],
        nfa: Program::default(),
        dfa: Program::default(), // Empty or non-matching DFA
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::new(vec![]),
        match_type: MatchType::Dfa,
    });

    let cache = RefCell::new(ProgramCacheInner::default());
    let executor = ExecNoSync { ro: &ro, cache: &cache };

    let result = executor.find_at(text, start);
}

#[test]
fn test_find_at_no_match_case_with_max_start() {
    let text: &[u8] = &[0, 1, 2, 3, 4, 5];
    let start = 5;

    let ro = Arc::new(ExecReadOnly {
        res: vec!["test_regex".to_string()],
        nfa: Program::default(),
        dfa: Program::default(), // Empty or non-matching DFA
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::new(vec![]),
        match_type: MatchType::Dfa,
    });

    let cache = RefCell::new(ProgramCacheInner::default());
    let executor = ExecNoSync { ro: &ro, cache: &cache };

    let result = executor.find_at(text, start);
}

