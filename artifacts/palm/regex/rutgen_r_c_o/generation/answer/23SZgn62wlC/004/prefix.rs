// Answer 0

#[test]
fn test_exec_dfa_reverse_suffix_case1() {
    let text: &[u8] = b"abcabc";
    let original_start: usize = 0;
    
    let suffixes = LiteralSearcher::suffixes(Literals::from(vec![b"c".to_vec()]));
    let ro = Arc::new(ExecReadOnly {
        res: vec!["abc".to_string()],
        nfa: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![],
            only_utf8: true,
            is_bytes: false,
            is_dfa: true,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: false,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::empty(),
            dfa_size_limit: 10,
        },
        dfa: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![],
            only_utf8: true,
            is_bytes: false,
            is_dfa: true,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: false,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::empty(),
            dfa_size_limit: 10,
        },
        dfa_reverse: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![],
            only_utf8: true,
            is_bytes: false,
            is_dfa: true,
            is_reverse: true,
            is_anchored_start: false,
            is_anchored_end: false,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::empty(),
            dfa_size_limit: 10,
        },
        suffixes,
        match_type: MatchType::default(),
    });
    
    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let exec_no_sync = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };
    
    exec_no_sync.exec_dfa_reverse_suffix(text, original_start);
}

#[test]
fn test_exec_dfa_reverse_suffix_case2() {
    let text: &[u8] = b"testest";
    let original_start: usize = 2;

    let suffixes = LiteralSearcher::suffixes(Literals::from(vec![b"est".to_vec()]));
    let ro = Arc::new(ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![],
            only_utf8: true,
            is_bytes: false,
            is_dfa: true,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: false,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::empty(),
            dfa_size_limit: 10,
        },
        dfa: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![],
            only_utf8: true,
            is_bytes: false,
            is_dfa: true,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: false,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::empty(),
            dfa_size_limit: 10,
        },
        dfa_reverse: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![],
            only_utf8: true,
            is_bytes: false,
            is_dfa: true,
            is_reverse: true,
            is_anchored_start: false,
            is_anchored_end: false,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::empty(),
            dfa_size_limit: 10,
        },
        suffixes,
        match_type: MatchType::default(),
    });

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let exec_no_sync = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };

    exec_no_sync.exec_dfa_reverse_suffix(text, original_start);
}

