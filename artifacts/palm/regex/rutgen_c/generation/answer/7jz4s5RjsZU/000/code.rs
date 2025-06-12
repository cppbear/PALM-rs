// Answer 0

#[test]
fn test_read_captures_at_zero_slots() {
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
    let result = exec.read_captures_at(&mut locs, b"test", 0);
    assert!(result.is_none());
}

#[test]
fn test_read_captures_at_two_slots() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["t".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Literal(MatchLiteralType::Unanchored),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let mut locs = Locations(vec![None, None]);
    let result = exec.read_captures_at(&mut locs, b"test", 0);
    assert!(result.is_some());
    let (start, end) = result.unwrap();
    assert_eq!(start, 0);
    assert_eq!(end, 4);
    assert_eq!(locs.0[0], Some(start));
    assert_eq!(locs.0[1], Some(end));
}

#[test]
fn test_read_captures_at_with_no_match() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["xyz".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Literal(MatchLiteralType::Unanchored),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let mut locs = Locations(vec![None, None]);
    let result = exec.read_captures_at(&mut locs, b"test", 0);
    assert!(result.is_none());
}

#[test]
fn test_read_captures_at_with_anchor_end_match() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["t".to_string()],
        nfa: Program::default(), // Modify this to create a valid program
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Nfa(MatchNfaType::Auto),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let mut locs = Locations(vec![None, None]);
    let result = exec.read_captures_at(&mut locs, b"test", 0);
    assert!(result.is_none());
}

