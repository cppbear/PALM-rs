// Answer 0

#[test]
fn test_exec_dfa_reverse_suffix_case_1() {
    let text = b"abc";
    let original_start = 0;

    let suffixes = LiteralSearcher::suffixes(Literals::empty());
    let dfa_reverse = Program {
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
        prefixes: suffixes.clone(),
        dfa_size_limit: 0,
    };

    let exec_read_only = ExecReadOnly {
        res: vec!["abc".to_string()],
        nfa: dfa_reverse.clone(),
        dfa: dfa_reverse.clone(),
        dfa_reverse,
        suffixes,
        match_type: MatchType::default(),
    };

    let cache = ProgramCache::default();
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &cache,
    };

    exec_no_sync.exec_dfa_reverse_suffix(text, original_start);
}

#[test]
fn test_exec_dfa_reverse_suffix_case_2() {
    let text = b"xyz";
    let original_start = 1;

    let suffixes = LiteralSearcher::suffixes(Literals::empty());
    let dfa_reverse = Program {
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
        prefixes: suffixes.clone(),
        dfa_size_limit: 0,
    };

    let exec_read_only = ExecReadOnly {
        res: vec!["xyz".to_string()],
        nfa: dfa_reverse.clone(),
        dfa: dfa_reverse.clone(),
        dfa_reverse,
        suffixes,
        match_type: MatchType::default(),
    };

    let cache = ProgramCache::default();
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &cache,
    };

    exec_no_sync.exec_dfa_reverse_suffix(text, original_start);
}

