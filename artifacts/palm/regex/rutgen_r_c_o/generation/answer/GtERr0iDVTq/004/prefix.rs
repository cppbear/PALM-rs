// Answer 0

#[test]
fn test_is_anchor_end_match_large_text_not_anchored() {
    let text: Vec<u8> = vec![b'a'; (1 << 20) + 1]; // Length is 1MB + 1 byte
    let ro = ExecReadOnly {
        res: vec![String::from("test")],
        nfa: Program {},
        dfa: Program {},
        dfa_reverse: Program {},
        suffixes: LiteralSearcher::empty(),
        match_type: MatchType::default(),
    };
    let exec = ExecNoSync {
        ro: Arc::new(ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    
    exec.is_anchor_end_match(&text);
}

#[test]
fn test_is_anchor_end_match_large_text_not_anchored_case2() {
    let text: Vec<u8> = vec![b'b'; (1 << 20) + 10]; // Length is 1MB + 10 bytes
    let ro = ExecReadOnly {
        res: vec![String::from("test")],
        nfa: Program {},
        dfa: Program {},
        dfa_reverse: Program {},
        suffixes: LiteralSearcher::empty(),
        match_type: MatchType::default(),
    };
    let exec = ExecNoSync {
        ro: Arc::new(ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    
    exec.is_anchor_end_match(&text);
}

#[test]
fn test_is_anchor_end_match_large_text_not_anchored_case3() {
    let text: Vec<u8> = vec![b'c'; (2 << 20) - 1]; // Length is 2MB - 1 byte
    let ro = ExecReadOnly {
        res: vec![String::from("test")],
        nfa: Program {},
        dfa: Program {},
        dfa_reverse: Program {},
        suffixes: LiteralSearcher::empty(),
        match_type: MatchType::default(),
    };
    let exec = ExecNoSync {
        ro: Arc::new(ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    
    exec.is_anchor_end_match(&text);
}

