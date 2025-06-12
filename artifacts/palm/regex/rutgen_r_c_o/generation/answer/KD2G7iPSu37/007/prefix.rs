// Answer 0

#[test]
fn test_is_match_at_dfa_suffix_no_match_1() {
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
        start: InstPtr::default(),
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
        res: vec!["test".into()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });

    let exec = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };

    let text: &[u8] = b"a"; // text does not match the expected suffix
    let start: usize = 0;

    exec.is_match_at(text, start);
}

#[test]
fn test_is_match_at_dfa_suffix_no_match_2() {
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
        start: InstPtr::default(),
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
        res: vec!["test".into()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });

    let exec = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };

    let text: &[u8] = b"abcde"; // text does not match the expected suffix
    let start: usize = 0;

    exec.is_match_at(text, start);
}

#[test]
fn test_is_match_at_dfa_suffix_no_match_3() {
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
        start: InstPtr::default(),
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
        res: vec!["test".into()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });

    let exec = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };

    let text: &[u8] = b"xyz"; // text does not match the expected suffix
    let start: usize = 0;

    exec.is_match_at(text, start);
}

#[test]
fn test_is_match_at_dfa_suffix_no_match_4() {
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
        start: InstPtr::default(),
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
        res: vec!["example".into()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });

    let exec = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };

    let text: &[u8] = b"teststring"; // text does not match the expected suffix
    let start: usize = 0;

    exec.is_match_at(text, start);
}

#[test]
fn test_is_match_at_dfa_suffix_no_match_5() {
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
        start: InstPtr::default(),
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
        res: vec!["sample".into()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaSuffix,
    });

    let exec = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };

    let text: &[u8] = b"abcdef"; // text does not match the expected suffix
    let start: usize = 0;

    exec.is_match_at(text, start);
}

