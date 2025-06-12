// Answer 0

#[test]
fn test_is_anchor_end_match_exact_boundary() {
    let long_text: Vec<u8> = vec![b'a'; 1 << 20]; // 1MB of 'a'
    
    let suffix = FreqyPacked {
        pat: vec![b'a'], // lcs of length 1
        char_len: 1,
        rare1: b'a',
        rare1i: 0,
        rare2: b'a',
        rare2i: 0,
    };
    
    let lcs = LiteralSearcher {
        complete: true,
        lcp: suffix.clone(),
        lcs: suffix,
        matcher: Matcher::Empty,
    };

    let ro = ExecReadOnly {
        res: vec![],
        nfa: Program::default(), // mock Program with is_anchored_end set to true
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: lcs,
        match_type: MatchType::default(),
    };
    
    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    let result = exec.is_anchor_end_match(&long_text);
}

#[test]
fn test_is_anchor_end_match_with_suffix() {
    let long_text: Vec<u8> = vec![b'b'; 1 << 20]; // 1MB of 'b'
    
    let suffix = FreqyPacked {
        pat: vec![b'b'], // lcs of length 1
        char_len: 1,
        rare1: b'b',
        rare1i: 0,
        rare2: b'b',
        rare2i: 0,
    };
    
    let lcs = LiteralSearcher {
        complete: true,
        lcp: suffix.clone(),
        lcs: suffix,
        matcher: Matcher::Empty,
    };

    let ro = ExecReadOnly {
        res: vec![],
        nfa: Program::default(), // mock Program with is_anchored_end set to true
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: lcs,
        match_type: MatchType::default(),
    };
    
    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    let result = exec.is_anchor_end_match(&long_text);
}

