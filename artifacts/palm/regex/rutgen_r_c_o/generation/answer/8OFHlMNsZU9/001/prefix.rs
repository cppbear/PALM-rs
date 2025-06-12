// Answer 0

#[test]
fn test_captures_nfa_with_match_valid_range() {
    let text: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    let match_start: usize = 5;
    let match_end: usize = 10;
    let slots: &mut [Slot] = &mut [Slot::default(); 20];
    
    let exec_read_only = ExecReadOnly {
        res: vec!["abc".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    };

    let program_cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &program_cache,
    };

    exec.captures_nfa_with_match(slots, text, match_start, match_end);
}

#[test]
fn test_captures_nfa_with_match_boundary() {
    let text: &[u8] = b"abcde";
    let match_start: usize = 0;
    let match_end: usize = 3;
    let slots: &mut [Slot] = &mut [Slot::default(); 6];
    
    let exec_read_only = ExecReadOnly {
        res: vec!["abc".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    };

    let program_cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &program_cache,
    };

    exec.captures_nfa_with_match(slots, text, match_start, match_end);
}

#[test]
#[should_panic]
fn test_captures_nfa_with_match_exceeding_limits() {
    let text: &[u8] = b"short";
    let match_start: usize = 0;
    let match_end: usize = 5; // This should trigger a panic because match_end + 2 > text.len()
    let slots: &mut [Slot] = &mut [Slot::default(); 10];
    
    let exec_read_only = ExecReadOnly {
        res: vec!["abc".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    };

    let program_cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &program_cache,
    };

    exec.captures_nfa_with_match(slots, text, match_start, match_end);
}

#[test]
fn test_captures_nfa_with_match_empty_text() {
    let text: &[u8] = b"";
    let match_start: usize = 0;
    let match_end: usize = 0;
    let slots: &mut [Slot] = &mut [Slot::default(); 6];
    
    let exec_read_only = ExecReadOnly {
        res: vec!["abc".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    };

    let program_cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &program_cache,
    };

    exec.captures_nfa_with_match(slots, text, match_start, match_end);
}

