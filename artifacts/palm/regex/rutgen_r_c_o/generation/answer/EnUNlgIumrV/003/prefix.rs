// Answer 0

#[test]
fn test_find_nfa_basic_case() {
    let text: &[u8] = b"abcde";
    let start = 0;

    let mut exec_no_sync = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![String::from("abc")],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    let result = exec_no_sync.find_nfa(MatchNfaType::Auto, text, start);
}

#[test]
fn test_find_nfa_with_different_start() {
    let text: &[u8] = b"abcabc";
    let start = 3;

    let mut exec_no_sync = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![String::from("abc")],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    let result = exec_no_sync.find_nfa(MatchNfaType::PikeVM, text, start);
}

#[test]
fn test_find_nfa_empty_string() {
    let text: &[u8] = b"";
    let start = 0;

    let mut exec_no_sync = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![String::from("")],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    let result = exec_no_sync.find_nfa(MatchNfaType::Backtrack, text, start);
}

#[test]
fn test_find_nfa_edge_case() {
    let text: &[u8] = b"aaabaa";
    let start = 1;

    let mut exec_no_sync = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![String::from("aa")],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    let result = exec_no_sync.find_nfa(MatchNfaType::Auto, text, start);
}

#[test]
fn test_find_nfa_large_input() {
    let text: &[u8] = b"abcdefghijklmnopqrstuvwxyz" as &[u8; 26];
    let start = 0;

    let mut exec_no_sync = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![String::from("defghijklmnop")],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    let result = exec_no_sync.find_nfa(MatchNfaType::PikeVM, text, start);
}

