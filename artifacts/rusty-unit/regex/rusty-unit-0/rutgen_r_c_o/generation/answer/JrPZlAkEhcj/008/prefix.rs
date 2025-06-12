// Answer 0

#[test]
fn test_shortest_match_at_case_1() {
    let text = b"example input string with multiple matches";
    let start = 0;
    
    let program_cache = RefCell::new(ProgramCacheInner {
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
        only_utf8: true,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 100,
    };
    
    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("example")],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });

    let exec_no_sync = ExecNoSync {
        ro: &ro,
        cache: &program_cache,
    };

    exec_no_sync.shortest_match_at(text, start);
}

#[test]
fn test_shortest_match_at_case_2() {
    let text = b"special test case with different content";
    let start = 10;

    let program_cache = RefCell::new(ProgramCacheInner {
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
        only_utf8: true,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 100,
    };

    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("testing")],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });

    let exec_no_sync = ExecNoSync {
        ro: &ro,
        cache: &program_cache,
    };

    exec_no_sync.shortest_match_at(text, start);
}

#[test]
fn test_shortest_match_at_case_3() {
    let text = b"Finding matches in a longer string to check";
    let start = 5;

    let program_cache = RefCell::new(ProgramCacheInner {
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
        only_utf8: true,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 100,
    };

    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("matches")],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });

    let exec_no_sync = ExecNoSync {
        ro: &ro,
        cache: &program_cache,
    };

    exec_no_sync.shortest_match_at(text, start);
}

