// Answer 0

#[test]
fn test_captures_nfa_type_auto_no_match() {
    let text: &[u8] = &[0; 10];
    let start = 0;
    let mut slots = [Slot::default(); 2];
    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec![String::from("test_regex")],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec_no_sync = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };
    exec_no_sync.captures_nfa_type(MatchNfaType::Auto, &mut slots, text, start);
}

#[test]
fn test_captures_nfa_type_backtrack_no_match() {
    let text: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let start = 0;
    let mut slots = [Slot::default(); 2];
    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec![String::from("another_regex")],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec_no_sync = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };
    exec_no_sync.captures_nfa_type(MatchNfaType::Backtrack, &mut slots, text, start);
}

#[test]
fn test_captures_nfa_type_pikevm_no_match() {
    let text: &[u8] = &[255; 10];
    let start = 0;
    let mut slots = [Slot::default(); 2];
    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec![String::from("different_regex")],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec_no_sync = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };
    exec_no_sync.captures_nfa_type(MatchNfaType::PikeVM, &mut slots, text, start);
}

