// Answer 0

#[test]
fn test_captures_nfa_type_backtrack() {
    struct TestText<'a> {
        text: &'a [u8],
    }
    impl RegularExpression for TestText<'_> {
        type Text = [u8];

        fn slots_len(&self) -> usize {
            2
        }
        
        fn locations(&self) -> Locations {
            Locations::default()
        }

        fn next_after_empty(&self, _text: &Self::Text, _i: usize) -> usize {
            0
        }

        fn shortest_match_at(&self, _text: &Self::Text, _start: usize) -> Option<usize> {
            Some(0)
        }

        fn is_match_at(&self, _text: &Self::Text, _start: usize) -> bool {
            true
        }

        fn find_at(&self, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> {
            Some((0, 1))
        }

        fn read_captures_at(&self, _locs: &mut Locations, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> {
            Some((0, 1))
        }
    }

    let exec_read_only = ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    };

    let cache = RefCell::new(ProgramCacheInner::default());
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &cache,
    };

    let mut slots = [None, None];
    let text = b"some test text";
    
    let result = exec_no_sync.captures_nfa_type(MatchNfaType::Backtrack, &mut slots, text, 0);
    assert_eq!(result, Some((0, 1)));
}

#[test]
fn test_captures_nfa_type_pikevm() {
    struct TestText<'a> {
        text: &'a [u8],
    }
    impl RegularExpression for TestText<'_> {
        type Text = [u8];

        fn slots_len(&self) -> usize {
            2
        }

        fn locations(&self) -> Locations {
            Locations::default()
        }

        fn next_after_empty(&self, _text: &Self::Text, _i: usize) -> usize {
            0
        }

        fn shortest_match_at(&self, _text: &Self::Text, _start: usize) -> Option<usize> {
            Some(0)
        }

        fn is_match_at(&self, _text: &Self::Text, _start: usize) -> bool {
            true
        }

        fn find_at(&self, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> {
            Some((0, 1))
        }

        fn read_captures_at(&self, _locs: &mut Locations, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> {
            Some((0, 1))
        }
    }

    let exec_read_only = ExecReadOnly {
        res: vec!["example".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    };

    let cache = RefCell::new(ProgramCacheInner::default());
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &cache,
    };

    let mut slots = [None, None];
    let text = b"another example";
    
    let result = exec_no_sync.captures_nfa_type(MatchNfaType::PikeVM, &mut slots, text, 0);
    assert_eq!(result, Some((0, 1)));
}

