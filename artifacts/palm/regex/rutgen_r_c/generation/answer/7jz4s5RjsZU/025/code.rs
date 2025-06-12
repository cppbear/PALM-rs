// Answer 0

#[test]
fn test_read_captures_at_slots_len_zero() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Nothing,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let mut locs = Locations(vec![]);
    let text: &[u8] = b"sample text";
    let result = exec.read_captures_at(&mut locs, text, 0);
    assert!(result.is_none());
}

#[test]
fn test_read_captures_at_slots_len_two() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["foo".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Nothing,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let mut locs = Locations(vec![None, None]);
    let text: &[u8] = b"sample text";
    let result = exec.read_captures_at(&mut locs, text, 0);
    assert!(result.is_none());
}

#[test]
fn test_read_captures_at_is_anchor_end_match_false() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["foo".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaAnchoredReverse,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let mut locs = Locations(vec![None, None]);
    let text: &[u8] = b"sample text";
    let result = exec.read_captures_at(&mut locs, text, 0);
    assert!(result.is_none());
}

