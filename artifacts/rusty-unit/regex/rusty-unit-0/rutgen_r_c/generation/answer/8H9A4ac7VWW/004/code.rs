// Answer 0

#[test]
fn test_find_dfa_reverse_suffix_success() {
    struct TestRegularExpression;

    impl RegularExpression for TestRegularExpression {
        type Text = [u8];

        fn slots_len(&self) -> usize {
            0
        }

        fn locations(&self) -> Locations {
            Locations::new()
        }

        fn next_after_empty(&self, _text: &Self::Text, _i: usize) -> usize {
            0
        }

        fn shortest_match_at(&self, _text: &Self::Text, _start: usize) -> Option<usize> {
            Some(3)
        }

        fn is_match_at(&self, _text: &Self::Text, _start: usize) -> bool {
            true
        }

        fn find_at(&self, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> {
            Some((0, 5))
        }

        fn read_captures_at(
            &self,
            _locs: &mut Locations,
            _text: &Self::Text,
            _start: usize,
        ) -> Option<(usize, usize)> {
            Some((0, 5))
        }
    }

    let program = Program {
        insts: Vec::new(),
        matches: Vec::new(),
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let exec_read_only = ExecReadOnly {
        res: vec!["abc".to_string()],
        nfa: program.clone(),
        dfa: program,
        dfa_reverse: program,
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::default(),
    };

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let exec = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &cache,
    };

    let text = b"abcde";
    let start = 0;

    let result = exec.find_dfa_reverse_suffix(text, start);
    if let dfa::Result::Match((start, end)) = result {
        assert_eq!(start, 0);
        assert_eq!(end, 5);
    } else {
        panic!("Expected a match result but did not get one.");
    }
}

#[test]
fn test_find_dfa_reverse_suffix_no_match() {
    struct TestRegularExpression;

    impl RegularExpression for TestRegularExpression {
        type Text = [u8];

        fn slots_len(&self) -> usize {
            0
        }

        fn locations(&self) -> Locations {
            Locations::new()
        }

        fn next_after_empty(&self, _text: &Self::Text, _i: usize) -> usize {
            0
        }

        fn shortest_match_at(&self, _text: &Self::Text, _start: usize) -> Option<usize> {
            None
        }

        fn is_match_at(&self, _text: &Self::Text, _start: usize) -> bool {
            false
        }

        fn find_at(&self, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> {
            None
        }

        fn read_captures_at(
            &self,
            _locs: &mut Locations,
            _text: &Self::Text,
            _start: usize,
        ) -> Option<(usize, usize)> {
            None
        }
    }

    let program = Program {
        insts: Vec::new(),
        matches: Vec::new(),
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let exec_read_only = ExecReadOnly {
        res: vec!["xyz".to_string()],
        nfa: program.clone(),
        dfa: program,
        dfa_reverse: program,
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::default(),
    };

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let exec = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &cache,
    };

    let text = b"abcde";
    let start = 0;

    let result = exec.find_dfa_reverse_suffix(text, start);
    assert_eq!(result, dfa::Result::Quit);
}

