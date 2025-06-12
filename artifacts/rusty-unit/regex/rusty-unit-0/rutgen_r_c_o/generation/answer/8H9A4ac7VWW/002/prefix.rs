// Answer 0

#[test]
fn test_find_dfa_reverse_suffix_match() {
    let text: &[u8] = &[0, 1, 2, 3];
    let start: usize = 2;

    let exec = ExecNoSync {
        ro: Arc::new(ExecReadOnly {
            res: vec!["test_regex".to_string()],
            nfa: Program {
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
                prefixes: LiteralSearcher::default(),
                dfa_size_limit: 1024,
            },
            dfa: Program {
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
                is_anchored_end: false,
                has_unicode_word_boundary: false,
                prefixes: LiteralSearcher::default(),
                dfa_size_limit: 1024,
            },
            dfa_reverse: Program {
                insts: vec![],
                matches: vec![],
                captures: vec![],
                capture_name_idx: Arc::new(HashMap::new()),
                start: 0,
                byte_classes: vec![],
                only_utf8: false,
                is_bytes: false,
                is_dfa: false,
                is_reverse: true,
                is_anchored_start: false,
                is_anchored_end: false,
                has_unicode_word_boundary: false,
                prefixes: LiteralSearcher::default(),
                dfa_size_limit: 1024,
            },
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner {
            pikevm: pikevm::Cache::default(),
            backtrack: backtrack::Cache::default(),
            dfa: dfa::Cache::default(),
            dfa_reverse: dfa::Cache::default(),
        }),
    };

    exec.find_dfa_reverse_suffix(text, start);
}

#[test]
fn test_find_dfa_reverse_suffix_no_match() {
    let text: &[u8] = &[0, 1, 2, 3];
    let start: usize = 3;

    let exec = ExecNoSync {
        ro: Arc::new(ExecReadOnly {
            res: vec!["another_test_regex".to_string()],
            nfa: Program {
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
                prefixes: LiteralSearcher::default(),
                dfa_size_limit: 1024,
            },
            dfa: Program {
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
                is_anchored_end: false,
                has_unicode_word_boundary: false,
                prefixes: LiteralSearcher::default(),
                dfa_size_limit: 1024,
            },
            dfa_reverse: Program {
                insts: vec![],
                matches: vec![],
                captures: vec![],
                capture_name_idx: Arc::new(HashMap::new()),
                start: 0,
                byte_classes: vec![],
                only_utf8: false,
                is_bytes: false,
                is_dfa: false,
                is_reverse: true,
                is_anchored_start: false,
                is_anchored_end: false,
                has_unicode_word_boundary: false,
                prefixes: LiteralSearcher::default(),
                dfa_size_limit: 1024,
            },
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner {
            pikevm: pikevm::Cache::default(),
            backtrack: backtrack::Cache::default(),
            dfa: dfa::Cache::default(),
            dfa_reverse: dfa::Cache::default(),
        }),
    };

    exec.find_dfa_reverse_suffix(text, start);
}

#[test]
#[should_panic]
fn test_find_dfa_reverse_suffix_panic() {
    let text: &[u8] = &[0, 1, 2, 3];
    let start: usize = 0;

    let exec = ExecNoSync {
        ro: Arc::new(ExecReadOnly {
            res: vec!["panic_test".to_string()],
            nfa: Program {
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
                prefixes: LiteralSearcher::default(),
                dfa_size_limit: 1024,
            },
            dfa: Program {
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
                is_anchored_end: false,
                has_unicode_word_boundary: false,
                prefixes: LiteralSearcher::default(),
                dfa_size_limit: 1024,
            },
            dfa_reverse: Program {
                insts: vec![],
                matches: vec![],
                captures: vec![],
                capture_name_idx: Arc::new(HashMap::new()),
                start: 0,
                byte_classes: vec![],
                only_utf8: false,
                is_bytes: false,
                is_dfa: false,
                is_reverse: true,
                is_anchored_start: false,
                is_anchored_end: false,
                has_unicode_word_boundary: false,
                prefixes: LiteralSearcher::default(),
                dfa_size_limit: 1024,
            },
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner {
            pikevm: pikevm::Cache::default(),
            backtrack: backtrack::Cache::default(),
            dfa: dfa::Cache::default(),
            dfa_reverse: dfa::Cache::default(),
        }),
    };

    exec.find_dfa_reverse_suffix(text, start);
}

