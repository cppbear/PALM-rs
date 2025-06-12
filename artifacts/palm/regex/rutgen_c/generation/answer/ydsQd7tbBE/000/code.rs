// Answer 0

#[test]
fn test_match_nfa_type_auto() {
    struct TestRegularExpression;

    impl RegularExpression for TestRegularExpression {
        type Text = [u8];

        fn slots_len(&self) -> usize {
            0
        }

        fn locations(&self) -> Locations {
            Locations::default()
        }

        fn next_after_empty(&self, text: &Self::Text, i: usize) -> usize {
            i + 1
        }

        fn shortest_match_at(&self, text: &Self::Text, start: usize) -> Option<usize> {
            if start < text.len() { Some(start) } else { None }
        }

        fn is_match_at(&self, text: &Self::Text, start: usize) -> bool {
            start < text.len()
        }

        fn find_at(&self, text: &Self::Text, start: usize) -> Option<(usize, usize)> {
            if start < text.len() {
                Some((start, text.len()))
            } else {
                None
            }
        }

        fn read_captures_at(&self, locs: &mut Locations, text: &Self::Text, start: usize) -> Option<(usize, usize)> {
            if start < text.len() {
                Some((start, start + 1))
            } else {
                None
            }
        }
    }

    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("test")],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let result = exec.match_nfa_type(MatchNfaType::Auto, b"test", 0);
    assert!(result);
}

#[test]
fn test_match_nfa_type_backtrack() {
    struct TestRegularExpression;

    impl RegularExpression for TestRegularExpression {
        type Text = [u8];

        fn slots_len(&self) -> usize {
            0
        }

        fn locations(&self) -> Locations {
            Locations::default()
        }

        fn next_after_empty(&self, text: &Self::Text, i: usize) -> usize {
            i + 1
        }

        fn shortest_match_at(&self, text: &Self::Text, start: usize) -> Option<usize> {
            if start < text.len() { Some(start) } else { None }
        }

        fn is_match_at(&self, text: &Self::Text, start: usize) -> bool {
            start < text.len()
        }

        fn find_at(&self, text: &Self::Text, start: usize) -> Option<(usize, usize)> {
            if start < text.len() {
                Some((start, text.len()))
            } else {
                None
            }
        }

        fn read_captures_at(&self, locs: &mut Locations, text: &Self::Text, start: usize) -> Option<(usize, usize)> {
            if start < text.len() {
                Some((start, start + 1))
            } else {
                None
            }
        }
    }

    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("test")],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let result = exec.match_nfa_type(MatchNfaType::Backtrack, b"test", 0);
    assert!(result);
}

#[test]
fn test_match_nfa_type_pikevm() {
    struct TestRegularExpression;

    impl RegularExpression for TestRegularExpression {
        type Text = [u8];

        fn slots_len(&self) -> usize {
            0
        }

        fn locations(&self) -> Locations {
            Locations::default()
        }

        fn next_after_empty(&self, text: &Self::Text, i: usize) -> usize {
            i + 1
        }

        fn shortest_match_at(&self, text: &Self::Text, start: usize) -> Option<usize> {
            if start < text.len() { Some(start) } else { None }
        }

        fn is_match_at(&self, text: &Self::Text, start: usize) -> bool {
            start < text.len()
        }

        fn find_at(&self, text: &Self::Text, start: usize) -> Option<(usize, usize)> {
            if start < text.len() {
                Some((start, text.len()))
            } else {
                None
            }
        }

        fn read_captures_at(&self, locs: &mut Locations, text: &Self::Text, start: usize) -> Option<(usize, usize)> {
            if start < text.len() {
                Some((start, start + 1))
            } else {
                None
            }
        }
    }

    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("test")],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let result = exec.match_nfa_type(MatchNfaType::PikeVM, b"test", 0);
    assert!(result);
}

