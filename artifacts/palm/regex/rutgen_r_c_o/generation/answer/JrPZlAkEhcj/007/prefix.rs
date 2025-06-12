// Answer 0

#[test]
fn test_shortest_match_at_with_dfa_suffix_no_match() {
    let text: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let start: usize = 0;
    
    let match_type = MatchType::DfaSuffix;

    let program = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let exec_read_only = ExecReadOnly {
        res: vec!["test_regex".into()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type,
    };

    let cache_inner = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    };

    let cache = RefCell::new(cache_inner);

    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &cache,
    };

    exec_no_sync.shortest_match_at(text, start);
}

#[test]
fn test_shortest_match_at_with_dfa_suffix_no_match_edge_case() {
    let text: &[u8] = &[1, 2, 3, 4, 5];
    let start: usize = 1;

    let match_type = MatchType::DfaSuffix;

    let program = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let exec_read_only = ExecReadOnly {
        res: vec!["test_regex".into()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type,
    };

    let cache_inner = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    };

    let cache = RefCell::new(cache_inner);

    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &cache,
    };

    exec_no_sync.shortest_match_at(text, start);
}

