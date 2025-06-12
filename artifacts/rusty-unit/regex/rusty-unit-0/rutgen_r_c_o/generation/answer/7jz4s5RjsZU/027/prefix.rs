// Answer 0

#[test]
fn test_read_captures_at_slots_len_0() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::new(),
        dfa: Program::new(),
        dfa_reverse: Program::new(),
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::Dfa,
    });
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let mut locations = Locations(vec![]);
    let text = b"sample text";
    let start = 0;
    exec.read_captures_at(&mut locations, text, start);
}

#[test]
fn test_read_captures_at_slots_len_2() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["sample".to_string()],
        nfa: Program::new(),
        dfa: Program::new(),
        dfa_reverse: Program::new(),
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::Dfa,
    });
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let mut locations = Locations(vec![None, None]);
    let text = b"sample text";
    let start = 0;
    exec.read_captures_at(&mut locations, text, start);
}

#[test]
fn test_read_captures_at_slots_len_greater_than_2() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["another sample".to_string()],
        nfa: Program::new(),
        dfa: Program::new(),
        dfa_reverse: Program::new(),
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::Dfa,
    });
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let mut locations = Locations(vec![None, None, None]);
    let text = b"another sample text";
    let start = 0;
    exec.read_captures_at(&mut locations, text, start);
}

#[test]
fn test_read_captures_at_is_anchor_end_match_true() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["end match".to_string()],
        nfa: Program::new(),
        dfa: Program::new(),
        dfa_reverse: Program::new(),
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::Dfa,
    });
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let mut locations = Locations(vec![None, None]);
    let text = b"text end match";
    let start = 0;
    exec.read_captures_at(&mut locations, text, start);
}

