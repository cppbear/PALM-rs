// Answer 0

#[test]
fn test_find_at_nothing_match_type() {
    let match_type = MatchType::Nothing;
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    
    let text = [0u8];
    let start = 0;
    
    let result = exec.find_at(&text, start);
}

#[test]
fn test_find_at_empty_text_nothing_match_type() {
    let match_type = MatchType::Nothing;
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type,
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let text: &[u8] = &[];
    let start = 0;

    let result = exec.find_at(text, start);
}

