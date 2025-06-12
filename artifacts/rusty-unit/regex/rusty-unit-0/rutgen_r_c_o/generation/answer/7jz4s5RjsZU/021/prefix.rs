// Answer 0

#[test]
fn test_read_captures_at_with_zero_slots() {
    let text: &[u8] = b"test";
    let start: usize = 0;
    let locs = &mut Locations(vec![]);
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    exec.read_captures_at(locs, text, start);
}

#[test]
fn test_read_captures_at_with_two_slots() {
    let text: &[u8] = b"example";
    let start: usize = 0;
    let locs = &mut Locations(vec![None, None]);
    let ro = Arc::new(ExecReadOnly {
        res: vec!["example".into()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    exec.read_captures_at(locs, text, start);
}

#[test]
fn test_read_captures_at_with_fallback_to_nfa() {
    let text: &[u8] = b"test string";
    let start: usize = 0;
    let locs = &mut Locations(vec![None, None, None]);
    let ro = Arc::new(ExecReadOnly {
        res: vec!["test".into()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    exec.read_captures_at(locs, text, start);
}

#[test]
fn test_read_captures_at_edge_case() {
    let text: &[u8] = b"";
    let start: usize = 0;
    let locs = &mut Locations(vec![None]);
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    exec.read_captures_at(locs, text, start);
}

#[test]
fn test_read_captures_at_nfa_match_with_end_anchor() {
    let text: &[u8] = b"abcdef";
    let start: usize = 0;
    let locs = &mut Locations(vec![None, None]);
    let ro = Arc::new(ExecReadOnly {
        res: vec!["abc".into()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    
    exec.read_captures_at(locs, text, start);
}

