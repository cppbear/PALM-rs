// Answer 0

#[test]
fn test_read_captures_at_slot_empty() {
    let text = b"test string";
    let start = 0;
    let locs = &mut Locations(vec![]);
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program { is_anchored_start: false, ..Default::default() },
        dfa: Program { is_anchored_start: false, ..Default::default() },
        dfa_reverse: Program { ..Default::default() },
        suffixes: LiteralSearcher::new(vec![]),
        match_type: MatchType::DfaAnchoredReverse,
    });
    let exec = ExecNoSync { ro: &ro, cache: &RefCell::new(ProgramCacheInner::default()) };
    
    let _ = exec.read_captures_at(locs, text, start);
}

#[test]
fn test_read_captures_at_slot_two() {
    let text = b"example string";
    let start = 0;
    let locs = &mut Locations(vec![None, None]);
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program { is_anchored_start: true, ..Default::default() },
        dfa: Program { is_anchored_start: true, ..Default::default() },
        dfa_reverse: Program { ..Default::default() },
        suffixes: LiteralSearcher::new(vec![]),
        match_type: MatchType::DfaAnchoredReverse,
    });
    let exec = ExecNoSync { ro: &ro, cache: &RefCell::new(ProgramCacheInner::default()) };
    
    let _ = exec.read_captures_at(locs, text, start);
}

#[test]
fn test_read_captures_at_slot_two_empty_captures() {
    let text = b"another test";
    let start = 0;
    let locs = &mut Locations(vec![None, None]);
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program { is_anchored_start: true, ..Default::default() },
        dfa: Program { is_anchored_start: false, ..Default::default() },
        dfa_reverse: Program { ..Default::default() },
        suffixes: LiteralSearcher::new(vec![]),
        match_type: MatchType::DfaAnchoredReverse,
    });
    let exec = ExecNoSync { ro: &ro, cache: &RefCell::new(ProgramCacheInner::default()) };
    
    let _ = exec.read_captures_at(locs, text, start);
}

#[test]
fn test_read_captures_at_quit_status() {
    let text = b"quit example";
    let start = 0;
    let locs = &mut Locations(vec![None, None]);
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program { is_anchored_start: true, ..Default::default() },
        dfa: Program { is_anchored_start: false, ..Default::default() },
        dfa_reverse: Program { ..Default::default() },
        suffixes: LiteralSearcher::new(vec![]),
        match_type: MatchType::DfaAnchoredReverse,
    });
    let exec = ExecNoSync { ro: &ro, cache: &RefCell::new(ProgramCacheInner::default()) };
    
    let _ = exec.read_captures_at(locs, text, start);
}

