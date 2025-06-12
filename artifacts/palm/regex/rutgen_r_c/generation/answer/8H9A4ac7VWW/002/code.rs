// Answer 0

#[test]
fn test_find_dfa_reverse_suffix_match() {
    struct TestRegex;
    
    impl RegularExpression for TestRegex {
        type Text = [u8];
        
        fn slots_len(&self) -> usize { 0 }
        fn locations(&self) -> Locations { Locations::default() }
        fn next_after_empty(&self, _text: &Self::Text, _i: usize) -> usize { 0 }
        fn shortest_match_at(&self, _text: &Self::Text, _start: usize) -> Option<usize> { None }
        fn is_match_at(&self, _text: &Self::Text, _start: usize) -> bool { false }
        fn find_at(&self, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> { None }
        fn read_captures_at(&self, _locs: &mut Locations, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> { None }
    }

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
        only_utf8: false,
        is_bytes: true,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };
    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program,
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let exec_no_sync = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };

    let text = b"test text";
    let start = 0;

    let result = exec_no_sync.find_dfa_reverse_suffix(text, start);
    assert!(matches!(result, dfa::Result::Quit));
}

#[test]
fn test_find_dfa_reverse_suffix_no_match() {
    struct TestRegex;

    impl RegularExpression for TestRegex {
        type Text = [u8];

        fn slots_len(&self) -> usize { 0 }
        fn locations(&self) -> Locations { Locations::default() }
        fn next_after_empty(&self, _text: &Self::Text, _i: usize) -> usize { 0 }
        fn shortest_match_at(&self, _text: &Self::Text, _start: usize) -> Option<usize> { None }
        fn is_match_at(&self, _text: &Self::Text, _start: usize) -> bool { false }
        fn find_at(&self, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> { None }
        fn read_captures_at(&self, _locs: &mut Locations, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> { None }
    }

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
        only_utf8: false,
        is_bytes: true,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };
    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program,
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let exec_no_sync = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };

    let text = b"no match here";
    let start = 0;

    let result = exec_no_sync.find_dfa_reverse_suffix(text, start);
    assert!(matches!(result, dfa::Result::Quit));
}

#[test]
#[should_panic(expected = "BUG: reverse match implies forward match")]
fn test_find_dfa_reverse_suffix_panic() {
    struct TestRegex;

    impl RegularExpression for TestRegex {
        type Text = [u8];

        fn slots_len(&self) -> usize { 0 }
        fn locations(&self) -> Locations { Locations::default() }
        fn next_after_empty(&self, _text: &Self::Text, _i: usize) -> usize { 0 }
        fn shortest_match_at(&self, _text: &Self::Text, _start: usize) -> Option<usize> { None }
        fn is_match_at(&self, _text: &Self::Text, _start: usize) -> bool { false }
        fn find_at(&self, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> { None }
        fn read_captures_at(&self, _locs: &mut Locations, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> { None }
    }

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
        only_utf8: false,
        is_bytes: true,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };
    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program,
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let exec_no_sync = ExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };

    let text = b"unexpected panic case";
    let start = 0;

    exec_no_sync.find_dfa_reverse_suffix(text, start);
}

