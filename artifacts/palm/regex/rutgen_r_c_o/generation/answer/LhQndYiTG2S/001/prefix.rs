// Answer 0

#[test]
fn test_find_literals_anchored_end_empty() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::empty(),
        match_type: MatchType::default(),
    });
    let exec = ExecNoSync { ro: &ro, cache: &RefCell::new(ProgramCacheInner::default()) };
    let text: &[u8] = b"";
    let start = 0;
    let result = exec.find_literals(MatchLiteralType::AnchoredEnd, text, start);
}

#[test]
fn test_find_literals_anchored_end_no_match() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::suffixes(Literals::from(vec![b"abc".to_vec()])),
        match_type: MatchType::default(),
    });
    let exec = ExecNoSync { ro: &ro, cache: &RefCell::new(ProgramCacheInner::default()) };
    let text: &[u8] = b"def";
    let start = 0;
    let result = exec.find_literals(MatchLiteralType::AnchoredEnd, text, start);
}

#[test]
fn test_find_literals_anchored_end_match_at_end() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::suffixes(Literals::from(vec![b"xyz".to_vec()])),
        match_type: MatchType::default(),
    });
    let exec = ExecNoSync { ro: &ro, cache: &RefCell::new(ProgramCacheInner::default()) };
    let text: &[u8] = b"some text ending in xyz";
    let start = text.len() - 3;
    let result = exec.find_literals(MatchLiteralType::AnchoredEnd, text, start);
}

#[test]
fn test_find_literals_anchored_end_match_at_start() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::suffixes(Literals::from(vec![b"start".to_vec()])),
        match_type: MatchType::default(),
    });
    let exec = ExecNoSync { ro: &ro, cache: &RefCell::new(ProgramCacheInner::default()) };
    let text: &[u8] = b"start";
    let start = 0;
    let result = exec.find_literals(MatchLiteralType::AnchoredEnd, text, start);
}

#[test]
fn test_find_literals_anchored_end_start_boundary() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::suffixes(Literals::from(vec![b"abc".to_vec()])),
        match_type: MatchType::default(),
    });
    let exec = ExecNoSync { ro: &ro, cache: &RefCell::new(ProgramCacheInner::default()) };
    let text: &[u8] = b"abc";
    let start = text.len() - 1;
    let result = exec.find_literals(MatchLiteralType::AnchoredEnd, text, start);
}

