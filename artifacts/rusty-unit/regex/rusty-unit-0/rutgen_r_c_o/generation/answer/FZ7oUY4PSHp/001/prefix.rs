// Answer 0

#[test]
fn test_shortest_dfa_empty_text_start_0() {
    let prog_cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });
    let ro = Arc::new(ExecReadOnly {
        res: vec!["a".to_string()],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 0 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 0 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 0 },
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let exec = ExecNoSync { ro: &ro, cache: &prog_cache };

    exec.shortest_dfa(&[], 0);
}

#[test]
fn test_shortest_dfa_single_byte_start_0() {
    let prog_cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });
    let ro = Arc::new(ExecReadOnly {
        res: vec!["a".to_string()],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 0 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 0 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 0 },
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let exec = ExecNoSync { ro: &ro, cache: &prog_cache };

    exec.shortest_dfa(&[97], 0); // ASCII for 'a'
}

#[test]
fn test_shortest_dfa_byte_range() {
    let prog_cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });
    let ro = Arc::new(ExecReadOnly {
        res: vec!["a".to_string()],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 0 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 0 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 0 },
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let exec = ExecNoSync { ro: &ro, cache: &prog_cache };

    exec.shortest_dfa(&[0, 1, 2, 3, 255], 0);
}

#[test]
fn test_shortest_dfa_specific_pattern() {
    let prog_cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });
    let ro = Arc::new(ExecReadOnly {
        res: vec!["abc".to_string()],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 0 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 0 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 0 },
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let exec = ExecNoSync { ro: &ro, cache: &prog_cache };

    exec.shortest_dfa(b"abc", 0);
}

#[test]
fn test_shortest_dfa_max_length_text() {
    let prog_cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });
    let ro = Arc::new(ExecReadOnly {
        res: vec!["a{1,5}".to_string()],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 0 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 0 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 0 },
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let exec = ExecNoSync { ro: &ro, cache: &prog_cache };

    exec.shortest_dfa(b"aaaaa", 0);
}

