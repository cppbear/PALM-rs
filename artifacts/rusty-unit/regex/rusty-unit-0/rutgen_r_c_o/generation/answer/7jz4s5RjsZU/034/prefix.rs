// Answer 0

#[test]
fn test_read_captures_at_empty_slots() {
    let text = b"";
    let mut locations = Locations(vec![]);
    let cache = RefCell::new(ProgramCacheInner::default());
    let ro = Arc::new(ExecReadOnly { 
        res: vec![], 
        nfa: Program::default(), 
        dfa: Program::default(), 
        dfa_reverse: Program::default(), 
        suffixes: LiteralSearcher::default(), 
        match_type: MatchType::Nothing 
    });
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    exec.read_captures_at(&mut locations, text, 0);
}

#[test]
fn test_read_captures_at_two_slots() {
    let text = b"abcde";
    let mut locations = Locations(vec![None, None]);
    let cache = RefCell::new(ProgramCacheInner::default());
    let ro = Arc::new(ExecReadOnly { 
        res: vec![], 
        nfa: Program::default(), 
        dfa: Program::default(), 
        dfa_reverse: Program::default(), 
        suffixes: LiteralSearcher::default(), 
        match_type: MatchType::Nothing 
    });
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    exec.read_captures_at(&mut locations, text, 0);
}

#[test]
fn test_read_captures_at_more_than_two_slots() {
    let text = b"abcdef";
    let mut locations = Locations(vec![None, None, None, None]);
    let cache = RefCell::new(ProgramCacheInner::default());
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Nothing 
    });
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    exec.read_captures_at(&mut locations, text, 0);
}

#[test]
fn test_read_captures_at_large_text() {
    let text = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
    let mut locations = Locations(vec![None, None, None, None, None]);
    let cache = RefCell::new(ProgramCacheInner::default());
    let ro = Arc::new(ExecReadOnly { 
        res: vec![], 
        nfa: Program::default(), 
        dfa: Program::default(), 
        dfa_reverse: Program::default(), 
        suffixes: LiteralSearcher::default(), 
        match_type: MatchType::Nothing 
    });
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    exec.read_captures_at(&mut locations, text, 0);
}

#[test]
fn test_read_captures_at_very_large_text() {
    let text = vec![b'a'; 1048576];
    let mut locations = Locations(vec![None, None, None]);
    let cache = RefCell::new(ProgramCacheInner::default());
    let ro = Arc::new(ExecReadOnly { 
        res: vec![], 
        nfa: Program::default(), 
        dfa: Program::default(), 
        dfa_reverse: Program::default(), 
        suffixes: LiteralSearcher::default(), 
        match_type: MatchType::Nothing 
    });
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    exec.read_captures_at(&mut locations, &text, 0);
}

