// Answer 0

#[test]
fn test_many_matches_at_literal() {
    use std::sync::Arc;
    use std::cell::RefCell;
    
    let literal_searcher = LiteralSearcher::new(vec![b'a', b'b']);
    let program = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: literal_searcher.clone(),
        dfa_size_limit: 0,
    };

    let exec_read_only = ExecReadOnly {
        res: vec![String::from("a")],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: literal_searcher,
        match_type: MatchType::Literal(MatchLiteralType::AnchoredEnd),
    };

    let program_cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &program_cache,
    };

    let mut matches = vec![false];
    let text = b"abc";
    let start = 0;
    
    // Test the function
    let result = exec_no_sync.many_matches_at(&mut matches, text, start);
    
    assert!(result);
    assert!(matches[0]);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_many_matches_at_literal_panic() {
    use std::sync::Arc;
    use std::cell::RefCell;
    
    let literal_searcher = LiteralSearcher::new(vec![b'a', b'b']);
    let program = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: literal_searcher.clone(),
        dfa_size_limit: 0,
    };

    let exec_read_only = ExecReadOnly {
        res: vec![String::from("a")],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: literal_searcher,
        match_type: MatchType::Literal(MatchLiteralType::AnchoredEnd),
    };

    let program_cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &program_cache,
    };

    let mut matches = vec![false];
    let text = b"xyz";
    let start = 0;

    // This should cause a panic due to the assertion in the test context
    exec_no_sync.many_matches_at(&mut matches, text, start);
}

