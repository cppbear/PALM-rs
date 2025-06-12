// Answer 0

#[test]
fn test_shortest_match_at_literal_success() {
    let text = b"hello world";
    let start = 0;
    
    let ro = Arc::new(ExecReadOnly {
        res: vec!["hello".to_string()],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: true, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: true, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Literal(MatchLiteralType::Unanchored),
    });

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let exec = ExecNoSync { ro: &ro, cache: &cache };
    
    let result = exec.shortest_match_at(text, start);
    assert_eq!(result, Some(5));
}

#[test]
fn test_shortest_match_at_literal_no_match() {
    let text = b"goodbye world";
    let start = 0;
    
    let ro = Arc::new(ExecReadOnly {
        res: vec!["hello".to_string()],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: true, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: true, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Literal(MatchLiteralType::Unanchored),
    });

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let exec = ExecNoSync { ro: &ro, cache: &cache };
    
    let result = exec.shortest_match_at(text, start);
    assert_eq!(result, None);
}

#[test]
fn test_shortest_match_at_dfa_match() {
    let text = b"example string";
    let start = 0;

    let ro = Arc::new(ExecReadOnly {
        res: vec!["example".to_string()],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: true, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: true, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Dfa,
    });

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let exec = ExecNoSync { ro: &ro, cache: &cache };
    
    let result = exec.shortest_match_at(text, start);
    assert_eq!(result, Some(7));
}

#[test]
fn test_shortest_match_at_nfa_no_match() {
    let text = b"no match here";
    let start = 0;

    let ro = Arc::new(ExecReadOnly {
        res: vec!["notfound".to_string()],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: true, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: true, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Nfa(MatchNfaType::Auto),
    });

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let exec = ExecNoSync { ro: &ro, cache: &cache };
    
    let result = exec.shortest_match_at(text, start);
    assert_eq!(result, None);
}

