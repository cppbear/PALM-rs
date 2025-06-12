// Answer 0

#[test]
fn test_read_captures_at_empty_slots() {
    let text: &[u8] = b"test string";
    let start: usize = 0;
    let mut locs = Locations(vec![]);
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaAnchoredReverse,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let _result = exec.read_captures_at(&mut locs, text, start);
}

#[test]
fn test_read_captures_at_two_slots() {
    let text: &[u8] = b"another test string";
    let start: usize = 0;
    let mut locs = Locations(vec![None, None]);
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaAnchoredReverse,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let _result = exec.read_captures_at(&mut locs, text, start);
}

#[test]
fn test_read_captures_at_is_anchor_end_match_false() {
    let text: &[u8] = b"sample text";
    let start: usize = 0;
    let mut locs = Locations(vec![None, None]);
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaAnchoredReverse,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let _result = exec.read_captures_at(&mut locs, text, start);
}

#[test]
fn test_read_captures_at_find_dfa_anchored_reverse_no_match() {
    let text: &[u8] = b"no match here";
    let start: usize = 0;
    let mut locs = Locations(vec![None, None]);
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaAnchoredReverse,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let _result = exec.read_captures_at(&mut locs, text, start);
}

