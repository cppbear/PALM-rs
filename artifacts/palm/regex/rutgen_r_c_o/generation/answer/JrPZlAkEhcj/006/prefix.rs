// Answer 0

#[test]
fn test_shortest_match_at_with_valid_input() {
    let text: &[u8] = &[0, 1, 2, 3, 4, 5];
    let start: usize = 0;

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

    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec![String::from("test")],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });

    let exec_nosync = ExecNoSync {
        ro: &exec_read_only,
        cache: &program_cache,
    };

    exec_nosync.shortest_match_at(text, start);
}

#[test]
fn test_shortest_match_at_with_non_zero_start() {
    let text: &[u8] = &[255, 254, 253, 252];
    let start: usize = 1;

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

    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec![String::from("example")],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });

    let exec_nosync = ExecNoSync {
        ro: &exec_read_only,
        cache: &program_cache,
    };

    exec_nosync.shortest_match_at(text, start);
}

#[test]
fn test_shortest_match_at_with_large_text() {
    let text: &[u8] = &[0; 1024]; // text with 1024 zeros
    let start: usize = 0;

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

    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec![String::from("large_text")],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });

    let exec_nosync = ExecNoSync {
        ro: &exec_read_only,
        cache: &program_cache,
    };

    exec_nosync.shortest_match_at(text, start);
}

#[test]
fn test_shortest_match_at_with_full_range() {
    let text: &[u8] = &(0..=255).collect::<Vec<u8>>();
    let start: usize = 0;

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

    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec![String::from("full_range")],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });

    let exec_nosync = ExecNoSync {
        ro: &exec_read_only,
        cache: &program_cache,
    };

    exec_nosync.shortest_match_at(text, start);
}

