// Answer 0

#[test]
fn test_read_captures_at_len_zero() {
    let text = b"";
    let locs = Locations(vec![]);
    let exec = ExecNoSync { 
        ro: &Arc::new(ExecReadOnly {
            res: vec!["".to_string()],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::DfaAnchoredReverse,
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec.read_captures_at(&mut locs, text, 0);
}

#[test]
fn test_read_captures_at_len_two() {
    let text = b"test";
    let locs = Locations(vec![None, None]);
    let exec = ExecNoSync { 
        ro: &Arc::new(ExecReadOnly {
            res: vec!["test".to_string()],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::DfaAnchoredReverse,
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec.read_captures_at(&mut locs, text, 0);
}

#[test]
fn test_read_captures_at_len_two_with_match() {
    let text = b"matching";
    let locs = Locations(vec![None, None]);
    let exec = ExecNoSync { 
        ro: &Arc::new(ExecReadOnly {
            res: vec!["match".to_string()],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::DfaAnchoredReverse,
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec.read_captures_at(&mut locs, text, 0);
}

#[test]
fn test_read_captures_at_is_anchor_end_match() {
    let text = b"end_match";
    let locs = Locations(vec![None, None, None]);
    let exec = ExecNoSync { 
        ro: &Arc::new(ExecReadOnly {
            res: vec!["match".to_string()],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::DfaAnchoredReverse,
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec.read_captures_at(&mut locs, text, 0);
}

#[test]
fn test_read_captures_at_various_lengths() {
    let text = b"test for various lengths in matching";
    let locs = Locations(vec![None, None, None]);
    let exec = ExecNoSync { 
        ro: &Arc::new(ExecReadOnly {
            res: vec!["lengths".to_string()],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::DfaAnchoredReverse,
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec.read_captures_at(&mut locs, text, 0);
}

#[test]
fn test_read_captures_at_edge_case() {
    let text = b"a";
    let locs = Locations(vec![None, None]);
    let exec = ExecNoSync { 
        ro: &Arc::new(ExecReadOnly {
            res: vec!["a".to_string()],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::DfaAnchoredReverse,
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec.read_captures_at(&mut locs, text, 0);
}

