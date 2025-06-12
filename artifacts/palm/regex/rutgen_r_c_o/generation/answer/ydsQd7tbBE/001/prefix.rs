// Answer 0

#[test]
fn test_match_nfa_type_auto_case() {
    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec![String::from("a*b")],
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
    let text = vec![97, 98, 97, 98]; // byte representation of "abab"
    let start = 0;
    exec.match_nfa_type(MatchNfaType::Auto, &text, start);
}

#[test]
fn test_match_nfa_type_backtrack_case() {
    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec![String::from("a*b")],
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
    let text = vec![97, 98, 97, 98]; // byte representation of "abab"
    let start = 0;
    exec.match_nfa_type(MatchNfaType::Backtrack, &text, start);
}

#[test]
fn test_match_nfa_type_pikevm_case() {
    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec![String::from("ab")],
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
    let text = vec![97, 98]; // byte representation of "ab"
    let start = 0;
    exec.match_nfa_type(MatchNfaType::PikeVM, &text, start);
}

#[test]
fn test_match_nfa_type_empty_text() {
    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec![String::from("a*b")],
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
    let text: Vec<u8> = vec![];
    let start = 0;
    exec.match_nfa_type(MatchNfaType::Auto, &text, start);
}

#[test]
fn test_match_nfa_type_start_out_of_bounds() {
    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec![String::from("a*b")],
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
    let text = vec![97, 98, 97, 98]; // byte representation of "abab"
    let start = 5; // out of bounds
    exec.match_nfa_type(MatchNfaType::Backtrack, &text, start);
}

