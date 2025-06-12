// Answer 0

#[test]
fn test_many_matches_at_case_1() {
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
        only_utf8: true,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };

    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec!["test_regex".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Dfa,
    });

    let exec_no_sync = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };

    let mut matches = vec![false];
    let text = &[1, 2, 3, 4, 5];
    let start = 0;

    exec_no_sync.many_matches_at(&mut matches, text, start);
}

#[test]
fn test_many_matches_at_case_2() {
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
        only_utf8: true,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };

    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec!["another_test_regex".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Dfa,
    });

    let exec_no_sync = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };

    let mut matches = vec![false];
    let text = &[5, 6, 7, 8, 9];
    let start = 0;

    exec_no_sync.many_matches_at(&mut matches, text, start);
}

#[test]
fn test_many_matches_at_case_3() {
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
        only_utf8: true,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };

    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec!["third_test_regex".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaMany,
    });

    let exec_no_sync = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };

    let mut matches = vec![false];
    let text = &[10, 11, 12, 13, 14];
    let start = 0;

    exec_no_sync.many_matches_at(&mut matches, text, start);
}

