// Answer 0

#[test]
fn test_match_nfa_type_auto_backtrack() {
    struct DummyText;

    impl RegularExpression for DummyText {
        type Text = [u8];

        fn slots_len(&self) -> usize { 0 }
        fn locations(&self) -> Locations { Locations::default() }
        fn next_after_empty(&self, text: &Self::Text, i: usize) -> usize { i }
        fn shortest_match_at(&self, text: &Self::Text, start: usize) -> Option<usize> { Some(start) }
        fn is_match_at(&self, text: &Self::Text, start: usize) -> bool { true }
        fn find_at(&self, text: &Self::Text, start: usize) -> Option<(usize, usize)> { Some((start, text.len())) }
        fn read_captures_at(&self, locs: &mut Locations, text: &Self::Text, start: usize) -> Option<(usize, usize)> { None }
    }

    let text = b"sample text";
    let mut exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly { 
            res: vec!["sample".to_string()], 
            nfa: Program::default(), 
            dfa: Program::default(), 
            dfa_reverse: Program::default(), 
            suffixes: LiteralSearcher::default(), 
            match_type: MatchNfaType::Auto,
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    assert_eq!(exec.match_nfa_type(MatchNfaType::Auto, text, 0), true);
}

#[test]
fn test_match_nfa_type_backtrack() {
    struct DummyText;

    impl RegularExpression for DummyText {
        type Text = [u8];

        fn slots_len(&self) -> usize { 0 }
        fn locations(&self) -> Locations { Locations::default() }
        fn next_after_empty(&self, text: &Self::Text, i: usize) -> usize { i }
        fn shortest_match_at(&self, text: &Self::Text, start: usize) -> Option<usize> { Some(start) }
        fn is_match_at(&self, text: &Self::Text, start: usize) -> bool { true }
        fn find_at(&self, text: &Self::Text, start: usize) -> Option<(usize, usize)> { Some((start, text.len())) }
        fn read_captures_at(&self, locs: &mut Locations, text: &Self::Text, start: usize) -> Option<(usize, usize)> { None }
    }

    let text = b"another example";
    let mut exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly { 
            res: vec!["another".to_string()], 
            nfa: Program::default(), 
            dfa: Program::default(), 
            dfa_reverse: Program::default(), 
            suffixes: LiteralSearcher::default(), 
            match_type: MatchNfaType::Backtrack,
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    assert_eq!(exec.match_nfa_type(MatchNfaType::Backtrack, text, 0), true);
}

#[test]
fn test_match_nfa_type_pikevm() {
    struct DummyText;

    impl RegularExpression for DummyText {
        type Text = [u8];

        fn slots_len(&self) -> usize { 0 }
        fn locations(&self) -> Locations { Locations::default() }
        fn next_after_empty(&self, text: &Self::Text, i: usize) -> usize { i }
        fn shortest_match_at(&self, text: &Self::Text, start: usize) -> Option<usize> { Some(start) }
        fn is_match_at(&self, text: &Self::Text, start: usize) -> bool { true }
        fn find_at(&self, text: &Self::Text, start: usize) -> Option<(usize, usize)> { Some((start, text.len())) }
        fn read_captures_at(&self, locs: &mut Locations, text: &Self::Text, start: usize) -> Option<(usize, usize)> { None }
    }

    let text = b"third test for pikevm";
    let mut exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly { 
            res: vec!["third".to_string()], 
            nfa: Program::default(), 
            dfa: Program::default(), 
            dfa_reverse: Program::default(), 
            suffixes: LiteralSearcher::default(), 
            match_type: MatchNfaType::PikeVM,
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    
    assert_eq!(exec.match_nfa_type(MatchNfaType::PikeVM, text, 0), true);
}

#[test]
#[should_panic]
fn test_match_nfa_type_out_of_bounds() {
    struct DummyText;

    impl RegularExpression for DummyText {
        type Text = [u8];

        fn slots_len(&self) -> usize { 0 }
        fn locations(&self) -> Locations { Locations::default() }
        fn next_after_empty(&self, text: &Self::Text, i: usize) -> usize { i }
        fn shortest_match_at(&self, text: &Self::Text, start: usize) -> Option<usize> { Some(start) }
        fn is_match_at(&self, text: &Self::Text, start: usize) -> bool { true }
        fn find_at(&self, text: &Self::Text, start: usize) -> Option<(usize, usize)> { Some((start, text.len())) }
        fn read_captures_at(&self, locs: &mut Locations, text: &Self::Text, start: usize) -> Option<(usize, usize)> { None }
    }

    let text = b"this text is valid";
    let mut exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly { 
            res: vec!["test".to_string()], 
            nfa: Program::default(), 
            dfa: Program::default(), 
            dfa_reverse: Program::default(), 
            suffixes: LiteralSearcher::default(), 
            match_type: MatchNfaType::Auto,
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    
    // This call should panic due to start being out of bounds
    exec.match_nfa_type(MatchNfaType::Auto, text, 100);
}

