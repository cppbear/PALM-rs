// Answer 0

#[test]
fn test_is_match_at_with_minimal_length() {
    let text: Vec<u8> = vec![b'a'];
    let start: usize = 0;
    let match_type = MatchType::DfaAnchoredReverse;

    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![String::from("a")],
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
                is_anchored_start: true,
                is_anchored_end: true,
                has_unicode_word_boundary: false,
                prefixes: LiteralSearcher::default(),
                dfa_size_limit: 0,
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
                is_anchored_start: true,
                is_anchored_end: true,
                has_unicode_word_boundary: false,
                prefixes: LiteralSearcher::default(),
                dfa_size_limit: 0,
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
                is_anchored_start: true,
                is_anchored_end: true,
                has_unicode_word_boundary: false,
                prefixes: LiteralSearcher::default(),
                dfa_size_limit: 0,
            },
            suffixes: LiteralSearcher::default(),
            match_type,
        }),
        cache: &RefCell::new(ProgramCacheInner {
            pikevm: pikevm::Cache::default(),
            backtrack: backtrack::Cache::default(),
            dfa: dfa::Cache::default(),
            dfa_reverse: dfa::Cache::default(),
        }),
    };

    exec_no_sync.is_match_at(&text, start);
}

#[test]
fn test_is_match_at_with_large_text() {
    let text: Vec<u8> = vec![b'a'; 1048576];
    let start: usize = 0;
    let match_type = MatchType::DfaAnchoredReverse;

    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![String::from("a")],
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
                is_anchored_start: true,
                is_anchored_end: true,
                has_unicode_word_boundary: false,
                prefixes: LiteralSearcher::default(),
                dfa_size_limit: 0,
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
                is_anchored_start: true,
                is_anchored_end: true,
                has_unicode_word_boundary: false,
                prefixes: LiteralSearcher::default(),
                dfa_size_limit: 0,
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
                is_anchored_start: true,
                is_anchored_end: true,
                has_unicode_word_boundary: false,
                prefixes: LiteralSearcher::default(),
                dfa_size_limit: 0,
            },
            suffixes: LiteralSearcher::default(),
            match_type,
        }),
        cache: &RefCell::new(ProgramCacheInner {
            pikevm: pikevm::Cache::default(),
            backtrack: backtrack::Cache::default(),
            dfa: dfa::Cache::default(),
            dfa_reverse: dfa::Cache::default(),
        }),
    };

    exec_no_sync.is_match_at(&text, start);
}

#[test]
fn test_is_match_at_with_non_empty_text() {
    let text: Vec<u8> = b"abcde".to_vec();
    let start: usize = 0;
    let match_type = MatchType::DfaAnchoredReverse;

    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![String::from("abc")],
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
                is_anchored_start: true,
                is_anchored_end: true,
                has_unicode_word_boundary: false,
                prefixes: LiteralSearcher::default(),
                dfa_size_limit: 0,
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
                is_anchored_start: true,
                is_anchored_end: true,
                has_unicode_word_boundary: false,
                prefixes: LiteralSearcher::default(),
                dfa_size_limit: 0,
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
                is_anchored_start: true,
                is_anchored_end: true,
                has_unicode_word_boundary: false,
                prefixes: LiteralSearcher::default(),
                dfa_size_limit: 0,
            },
            suffixes: LiteralSearcher::default(),
            match_type,
        }),
        cache: &RefCell::new(ProgramCacheInner {
            pikevm: pikevm::Cache::default(),
            backtrack: backtrack::Cache::default(),
            dfa: dfa::Cache::default(),
            dfa_reverse: dfa::Cache::default(),
        }),
    };

    exec_no_sync.is_match_at(&text, start);
}

