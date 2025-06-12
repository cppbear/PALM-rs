// Answer 0

#[test]
fn test_is_match_at_case1() {
    let regex_string = "a";
    let text: &[u8] = b"a"; 
    let start = 0;

    let match_type = MatchType::DfaAnchoredReverse;
    let dfa_reverse_program = Program {
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
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let ro = Arc::new(ExecReadOnly {
        res: vec![regex_string.to_string()],
        nfa: dfa_reverse_program.clone(),
        dfa: dfa_reverse_program.clone(),
        dfa_reverse: dfa_reverse_program,
        suffixes: LiteralSearcher::new(),
        match_type,
    });

    let exec_no_sync = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };

    exec_no_sync.is_match_at(text, start);
}

#[test]
fn test_is_match_at_case2() {
    let regex_string = "abc";
    let text: &[u8] = b"abc"; 
    let start = 0;

    let match_type = MatchType::DfaAnchoredReverse;
    let dfa_reverse_program = Program {
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
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let ro = Arc::new(ExecReadOnly {
        res: vec![regex_string.to_string()],
        nfa: dfa_reverse_program.clone(),
        dfa: dfa_reverse_program.clone(),
        dfa_reverse: dfa_reverse_program,
        suffixes: LiteralSearcher::new(),
        match_type,
    });

    let exec_no_sync = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };

    exec_no_sync.is_match_at(text, start);
}

#[test]
fn test_is_match_at_case3() {
    let regex_string = "abcd";
    let text: &[u8] = b"abcd"; 
    let start = 0;

    let match_type = MatchType::DfaAnchoredReverse;
    let dfa_reverse_program = Program {
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
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let ro = Arc::new(ExecReadOnly {
        res: vec![regex_string.to_string()],
        nfa: dfa_reverse_program.clone(),
        dfa: dfa_reverse_program.clone(),
        dfa_reverse: dfa_reverse_program,
        suffixes: LiteralSearcher::new(),
        match_type,
    });

    let exec_no_sync = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };

    exec_no_sync.is_match_at(text, start);
}

#[test]
fn test_is_match_at_case4() {
    let regex_string = "a";
    let text: &[u8] = b"abcde"; 
    let start = 0;

    let match_type = MatchType::DfaAnchoredReverse;
    let dfa_reverse_program = Program {
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
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let ro = Arc::new(ExecReadOnly {
        res: vec![regex_string.to_string()],
        nfa: dfa_reverse_program.clone(),
        dfa: dfa_reverse_program.clone(),
        dfa_reverse: dfa_reverse_program,
        suffixes: LiteralSearcher::new(),
        match_type,
    });

    let exec_no_sync = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };

    exec_no_sync.is_match_at(text, start);
}

#[test]
fn test_is_match_at_case5() {
    let regex_string = "xyz";
    let text: &[u8] = b"xyzxyz"; 
    let start = 0;

    let match_type = MatchType::DfaAnchoredReverse;
    let dfa_reverse_program = Program {
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
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let ro = Arc::new(ExecReadOnly {
        res: vec![regex_string.to_string()],
        nfa: dfa_reverse_program.clone(),
        dfa: dfa_reverse_program.clone(),
        dfa_reverse: dfa_reverse_program,
        suffixes: LiteralSearcher::new(),
        match_type,
    });

    let exec_no_sync = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };

    exec_no_sync.is_match_at(text, start);
}

