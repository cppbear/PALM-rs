// Answer 0

#[test]
fn test_many_matches_at_literal_match() {
    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });
    
    let program = Program {
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
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };
    
    let exec_ro = ExecReadOnly {
        res: vec!["abcde".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Literal(MatchLiteralType::Unanchored),
    };
    
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_ro),
        cache: &cache,
    };
    
    let mut matches = vec![false];
    let text = &[b'a', b'b', b'c', b'd', b'e'];

    exec_no_sync.many_matches_at(&mut matches, text, 0);
}

#[test]
fn test_many_matches_at_literal_no_match() {
    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });
    
    let program = Program {
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
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };
    
    let exec_ro = ExecReadOnly {
        res: vec!["abcde".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Literal(MatchLiteralType::Unanchored),
    };
    
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_ro),
        cache: &cache,
    };
    
    let mut matches = vec![false];
    let text = &[b'x', b'y', b'z'];

    exec_no_sync.many_matches_at(&mut matches, text, 0);
}

#[test]
fn test_many_matches_at_literal_empty_text() {
    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });
    
    let program = Program {
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
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };
    
    let exec_ro = ExecReadOnly {
        res: vec!["abcde".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Literal(MatchLiteralType::Unanchored),
    };
    
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_ro),
        cache: &cache,
    };
    
    let mut matches = vec![false];
    let text: &[u8] = &[];

    exec_no_sync.many_matches_at(&mut matches, text, 0);
}

