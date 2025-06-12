// Answer 0

#[test]
fn test_read_captures_at_empty_slots() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaMany,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let mut locs = Locations(vec![]);
    let text: &[u8] = b"some input text";
    exec.read_captures_at(&mut locs, text, 0);
}

#[test]
fn test_read_captures_at_two_slots() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaMany,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let mut locs = Locations(vec![None, None]);
    let text: &[u8] = b"some input text";
    exec.read_captures_at(&mut locs, text, 0);
}

#[test]
fn test_read_captures_at_slots_nonzero() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaMany,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let mut locs = Locations(vec![None, None, None]);
    let text: &[u8] = b"some input text";
    exec.read_captures_at(&mut locs, text, 0);
}

#[test]
#[should_panic]
fn test_read_captures_at_panic_dfa_many() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaMany,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let text: &[u8] = b"some input text";
    exec.read_captures_at(&mut Locations(vec![None]), text, 0);
}

#[test]
fn test_read_captures_at_slots_no_match() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaMany,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let mut locs = Locations(vec![None, None]);
    let text: &[u8] = b"";
    exec.read_captures_at(&mut locs, text, 0);
}

