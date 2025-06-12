// Answer 0

#[test]
fn test_into_byte_regex_set_empty() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec![],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::default(),
        }),
        cache: CachedThreadLocal::new(),
    };

    let regex_set = exec.into_byte_regex_set();
    assert_eq!(regex_set.len(), 0);
}

#[test]
fn test_into_byte_regex_set_single_regex() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec!["a".to_string()],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::default(),
        }),
        cache: CachedThreadLocal::new(),
    };

    let regex_set = exec.into_byte_regex_set();
    assert_eq!(regex_set.len(), 1);
}

#[test]
fn test_into_byte_regex_set_multiple_regexes() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::default(),
        }),
        cache: CachedThreadLocal::new(),
    };

    let regex_set = exec.into_byte_regex_set();
    assert_eq!(regex_set.len(), 3);
}

#[test]
fn test_into_byte_regex_set_with_complex_regex() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec![r"a.*b".to_string(), r"c?d".to_string()],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::default(),
        }),
        cache: CachedThreadLocal::new(),
    };

    let regex_set = exec.into_byte_regex_set();
    assert_eq!(regex_set.len(), 2);
}

