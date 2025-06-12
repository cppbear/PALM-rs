// Answer 0

#[test]
fn test_reverse_no_match_at_dead_state() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let mut cache = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    };
    let cache_ref = &mut cache;
    
    let result = Fsm::reverse(&prog, &cache_ref, false, b"example", 0);
    
    match result {
        Result::NoMatch(at) => assert_eq!(at, 0),
        _ => panic!("Expected NoMatch at position 0"),
    }
}

#[test]
fn test_reverse_match_found() {
    let prog = Program {
        insts: vec![],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let mut cache = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    };
    
    let cache_ref = &mut cache;

    let result = Fsm::reverse(&prog, &cache_ref, true, b"example", 7);
    
    match result {
        Result::Match(at) => assert_eq!(at, 8),
        _ => panic!("Expected Match at position 8"),
    }
}

#[test]
fn test_reverse_no_match_at_quit() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: true,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let mut cache = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    };
    
    let cache_ref = &mut cache;

    let result = Fsm::reverse(&prog, &cache_ref, false, b"test", 4);

    match result {
        Result::Quit => (),
        _ => panic!("Expected Quit result"),
    }
}

