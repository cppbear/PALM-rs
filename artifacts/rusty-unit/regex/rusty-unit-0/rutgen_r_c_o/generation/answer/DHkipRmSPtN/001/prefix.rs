// Answer 0

#[test]
fn test_program_cache_inner_new_empty_nfa() {
    let nfa = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };
    
    let ro = ExecReadOnly {
        res: vec!["".to_string()],
        nfa,
        dfa: nfa.clone(),
        dfa_reverse: nfa.clone(),
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::default(),
    };

    let _ = ProgramCacheInner::new(&ro);
}

#[test]
fn test_program_cache_inner_new_small_nfa() {
    let nfa = Program {
        insts: vec![1, 2],
        matches: vec![1],
        captures: vec![Some("group1".to_string())],
        capture_name_idx: Arc::new(HashMap::from([("group1".to_string(), 0)])),
        start: 0,
        byte_classes: vec![0, 1],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };
    
    let ro = ExecReadOnly {
        res: vec!["abc".to_string()],
        nfa,
        dfa: nfa.clone(),
        dfa_reverse: nfa.clone(),
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::default(),
    };

    let _ = ProgramCacheInner::new(&ro);
}

#[test]
fn test_program_cache_inner_new_large_nfa() {
    let nfa = Program {
        insts: (0..1000).collect(),
        matches: vec![0; 10],
        captures: vec![Some("group1".to_string()), Some("group2".to_string())],
        capture_name_idx: Arc::new(HashMap::from([
            ("group1".to_string(), 0),
            ("group2".to_string(), 1),
        ])),
        start: 0,
        byte_classes: vec![2, 3, 4],
        only_utf8: true,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };

    let ro = ExecReadOnly {
        res: vec!["pattern".to_string()],
        nfa,
        dfa: nfa.clone(),
        dfa_reverse: nfa.clone(),
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::default(),
    };

    let _ = ProgramCacheInner::new(&ro);
}

#[test]
fn test_program_cache_inner_new_dfa_enabled() {
    let nfa = Program {
        insts: vec![1, 2, 3],
        matches: vec![1],
        captures: vec![None],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![1, 2],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: true,
        is_anchored_end: true,
        has_unicode_word_boundary: true,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 50,
    };

    let ro = ExecReadOnly {
        res: vec!["test".to_string()],
        nfa,
        dfa: nfa.clone(),
        dfa_reverse: nfa.clone(),
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::default(),
    };

    let _ = ProgramCacheInner::new(&ro);
}

