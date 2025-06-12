// Answer 0

#[test]
fn test_program_cache_inner_new() {
    struct TestProgram {
        insts: Vec<u8>,
        matches: Vec<u8>,
        captures: Vec<Option<String>>,
        capture_name_idx: Arc<HashMap<String, usize>>,
        start: u8,
        byte_classes: Vec<u8>,
        only_utf8: bool,
        is_bytes: bool,
        is_dfa: bool,
        is_reverse: bool,
        is_anchored_start: bool,
        is_anchored_end: bool,
        has_unicode_word_boundary: bool,
        prefixes: LiteralSearcher,
        dfa_size_limit: usize,
    }

    let program = TestProgram {
        insts: vec![1, 2, 3],
        matches: vec![0],
        captures: vec![Some("group1".to_string())],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![b'a', b'b'],
        only_utf8: true,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let exec_read_only = ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: Program {
            insts: program.insts.clone(),
            matches: program.matches.clone(),
            captures: program.captures.clone(),
            capture_name_idx: program.capture_name_idx.clone(),
            start: program.start,
            byte_classes: program.byte_classes.clone(),
            only_utf8: program.only_utf8,
            is_bytes: program.is_bytes,
            is_dfa: program.is_dfa,
            is_reverse: program.is_reverse,
            is_anchored_start: program.is_anchored_start,
            is_anchored_end: program.is_anchored_end,
            has_unicode_word_boundary: program.has_unicode_word_boundary,
            prefixes: program.prefixes.clone(),
            dfa_size_limit: program.dfa_size_limit,
        },
        dfa: Program {
            insts: program.insts.clone(),
            matches: program.matches.clone(),
            captures: program.captures.clone(),
            capture_name_idx: program.capture_name_idx.clone(),
            start: program.start,
            byte_classes: program.byte_classes.clone(),
            only_utf8: program.only_utf8,
            is_bytes: program.is_bytes,
            is_dfa: true,
            is_reverse: program.is_reverse,
            is_anchored_start: program.is_anchored_start,
            is_anchored_end: program.is_anchored_end,
            has_unicode_word_boundary: program.has_unicode_word_boundary,
            prefixes: program.prefixes.clone(),
            dfa_size_limit: program.dfa_size_limit,
        },
        dfa_reverse: Program {
            insts: program.insts.clone(),
            matches: program.matches.clone(),
            captures: program.captures.clone(),
            capture_name_idx: program.capture_name_idx.clone(),
            start: program.start,
            byte_classes: program.byte_classes.clone(),
            only_utf8: program.only_utf8,
            is_bytes: program.is_bytes,
            is_dfa: program.is_dfa,
            is_reverse: true,
            is_anchored_start: program.is_anchored_start,
            is_anchored_end: program.is_anchored_end,
            has_unicode_word_boundary: program.has_unicode_word_boundary,
            prefixes: program.prefixes.clone(),
            dfa_size_limit: program.dfa_size_limit,
        },
        suffixes: program.prefixes.clone(),
        match_type: MatchType::default(),
    };

    let program_cache_inner = ProgramCacheInner::new(&exec_read_only);

    assert!(!program_cache_inner.pikevm.jobs.is_empty());
    assert!(!program_cache_inner.backtrack.jobs.is_empty());
    assert!(!program_cache_inner.dfa.jobs.is_empty());
    assert!(!program_cache_inner.dfa_reverse.jobs.is_empty());
}

