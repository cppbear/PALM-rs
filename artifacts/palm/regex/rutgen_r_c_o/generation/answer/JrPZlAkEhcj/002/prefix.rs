// Answer 0

#[test]
fn test_shortest_match_at_literal() {
    let text: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let start: usize = 0;
    let ro = Arc::new(ExecReadOnly {
        res: vec!["test".into()],
        nfa: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![Some("".into())],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![],
            only_utf8: true,
            is_bytes: false,
            is_dfa: true,
            is_reverse: false,
            is_anchored_start: true,
            is_anchored_end: true,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::default(),
            dfa_size_limit: 100,
        },
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Nfa(MatchNfaType::Auto),
    });
    let cache: ProgramCache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    
    let _ = exec.shortest_match_at(text, start);
}

#[test]
fn test_shortest_match_at_nfa() {
    let text: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let start: usize = 5;
    let ro = Arc::new(ExecReadOnly {
        res: vec!["example".into()],
        nfa: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![Some("".into())],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![],
            only_utf8: true,
            is_bytes: false,
            is_dfa: true,
            is_reverse: false,
            is_anchored_start: true,
            is_anchored_end: true,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::default(),
            dfa_size_limit: 100,
        },
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Nfa(MatchNfaType::Auto),
    });
    let cache: ProgramCache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    
    let _ = exec.shortest_match_at(text, start);
}

#[test]
fn test_shortest_match_at_start_boundary() {
    let text: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
    let start: usize = 0;
    let ro = Arc::new(ExecReadOnly {
        res: vec!["start_boundary".into()],
        nfa: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![Some("".into())],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![],
            only_utf8: true,
            is_bytes: false,
            is_dfa: true,
            is_reverse: false,
            is_anchored_start: true,
            is_anchored_end: true,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::default(),
            dfa_size_limit: 100,
        },
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Nfa(MatchNfaType::Auto),
    });
    let cache: ProgramCache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let _ = exec.shortest_match_at(text, start);
}

#[test]
fn test_shortest_match_at_end_boundary() {
    let text: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let start: usize = 9;
    let ro = Arc::new(ExecReadOnly {
        res: vec!["end_boundary".into()],
        nfa: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![Some("".into())],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![],
            only_utf8: true,
            is_bytes: false,
            is_dfa: true,
            is_reverse: false,
            is_anchored_start: true,
            is_anchored_end: true,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::default(),
            dfa_size_limit: 100,
        },
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Nfa(MatchNfaType::Auto),
    });
    let cache: ProgramCache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let _ = exec.shortest_match_at(text, start);
}

