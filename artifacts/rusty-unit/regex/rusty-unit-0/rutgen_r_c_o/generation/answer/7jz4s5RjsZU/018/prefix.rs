// Answer 0

#[test]
fn test_read_captures_at_empty_slots() {
    let text = b"";
    let start = 0;
    let locs = &mut Locations(vec![None; 0]); 
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Nothing,
    });
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    exec.read_captures_at(locs, text, start);
}

#[test]
fn test_read_captures_at_two_slots() {
    let text = b"abc";
    let start = 0;
    let locs = &mut Locations(vec![None, None]);
    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("abc")],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Nothing,
    });
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    exec.read_captures_at(locs, text, start);
}

#[test]
fn test_read_captures_at_multiple_slots() {
    let text = b"hello";
    let start = 0;
    let locs = &mut Locations(vec![None; 3]); 
    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("hello")],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Nothing,
    });
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    exec.read_captures_at(locs, text, start);
}

