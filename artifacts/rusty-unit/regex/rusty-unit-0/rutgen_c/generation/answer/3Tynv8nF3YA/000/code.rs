// Answer 0

#[test]
fn test_match_nfa_success() {
    struct MockRegularExpression;
    impl RegularExpression for MockRegularExpression {
        type Text = [u8];
        fn slots_len(&self) -> usize { 0 }
        fn locations(&self) -> Locations { Locations::default() }
        fn next_after_empty(&self, _text: &Self::Text, _i: usize) -> usize { 0 }
        fn shortest_match_at(&self, _text: &Self::Text, _start: usize) -> Option<usize> { Some(0) }
        fn is_match_at(&self, _text: &Self::Text, _start: usize) -> bool { true }
        fn find_at(&self, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> { Some((0, 1)) }
        fn read_captures_at(&self, _locs: &mut Locations, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> { None }
    }
    
    let exec_read_only = ExecReadOnly {
        res: vec!["a".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Auto,
    };

    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    assert!(exec_no_sync.match_nfa(b"a", 0));
}

#[test]
fn test_match_nfa_failure() {
    struct MockRegularExpression;
    impl RegularExpression for MockRegularExpression {
        type Text = [u8];
        fn slots_len(&self) -> usize { 0 }
        fn locations(&self) -> Locations { Locations::default() }
        fn next_after_empty(&self, _text: &Self::Text, _i: usize) -> usize { 0 }
        fn shortest_match_at(&self, _text: &Self::Text, _start: usize) -> Option<usize> { None }
        fn is_match_at(&self, _text: &Self::Text, _start: usize) -> bool { false }
        fn find_at(&self, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> { None }
        fn read_captures_at(&self, _locs: &mut Locations, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> { None }
    }

    let exec_read_only = ExecReadOnly {
        res: vec!["b".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Auto,
    };

    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    assert!(!exec_no_sync.match_nfa(b"a", 0));
}

