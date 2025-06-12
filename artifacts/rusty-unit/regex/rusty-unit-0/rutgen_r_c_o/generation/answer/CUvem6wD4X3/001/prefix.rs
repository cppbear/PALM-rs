// Answer 0

#[test]
fn test_into_byte_regex_empty_vec() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: Vec::new(),
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::new(),
            match_type: MatchType::default(),
        }),
        cache: CachedThreadLocal::new(),
    };
    exec.into_byte_regex();
}

#[test]
fn test_into_byte_regex_single_short_string() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec![String::from("a")],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::new(),
            match_type: MatchType::default(),
        }),
        cache: CachedThreadLocal::new(),
    };
    exec.into_byte_regex();
}

#[test]
fn test_into_byte_regex_multiple_short_strings() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec![String::from("abc"), String::from("def")],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::new(),
            match_type: MatchType::default(),
        }),
        cache: CachedThreadLocal::new(),
    };
    exec.into_byte_regex();
}

#[test]
fn test_into_byte_regex_maximum_elements() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: (0..100).map(|i| String::from_utf8(vec![b'a'; 255]).unwrap()).collect(),
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::new(),
            match_type: MatchType::default(),
        }),
        cache: CachedThreadLocal::new(),
    };
    exec.into_byte_regex();
}

#[test]
fn test_into_byte_regex_single_long_string() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec![String::from_utf8(vec![b'x'; 255]).unwrap()],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::new(),
            match_type: MatchType::default(),
        }),
        cache: CachedThreadLocal::new(),
    };
    exec.into_byte_regex();
}

#[test]
fn test_into_byte_regex_multiple_long_strings() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec![String::from_utf8(vec![b'y'; 200]).unwrap(), String::from_utf8(vec![b'z'; 200]).unwrap()],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::new(),
            match_type: MatchType::default(),
        }),
        cache: CachedThreadLocal::new(),
    };
    exec.into_byte_regex();
}

