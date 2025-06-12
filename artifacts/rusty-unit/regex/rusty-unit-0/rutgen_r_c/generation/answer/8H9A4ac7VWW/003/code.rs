// Answer 0

fn test_find_dfa_reverse_suffix_success() {
    struct MyRegex;

    impl RegularExpression for MyRegex {
        type Text = [u8];

        fn slots_len(&self) -> usize { 0 }
        fn locations(&self) -> Locations { Locations::default() }
        fn next_after_empty(&self, _text: &Self::Text, _i: usize) -> usize { 0 }
        fn shortest_match_at(&self, _text: &Self::Text, _start: usize) -> Option<usize> { Some(1) }
        fn is_match_at(&self, _text: &Self::Text, _start: usize) -> bool { true }
        fn find_at(&self, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> { Some((0, 1)) }
        fn read_captures_at(&self, _locs: &mut Locations, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> { Some((0, 1)) }
    }

    let ro = Arc::new(ExecReadOnly {
        res: vec!["a".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });

    let cache = RefCell::new(ProgramCacheInner::default());

    let exec = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };

    let result = exec.find_dfa_reverse_suffix(b"abc", 0);
    match result {
        dfa::Result::Match((start, end)) => {
            assert_eq!(start, 0);
            assert_eq!(end, 1);
        },
        _ => panic!("Expected a match result"),
    }
}

fn test_find_dfa_reverse_suffix_none() {
    struct MyRegex;

    impl RegularExpression for MyRegex {
        type Text = [u8];

        fn slots_len(&self) -> usize { 0 }
        fn locations(&self) -> Locations { Locations::default() }
        fn next_after_empty(&self, _text: &Self::Text, _i: usize) -> usize { 0 }
        fn shortest_match_at(&self, _text: &Self::Text, _start: usize) -> Option<usize> { None }
        fn is_match_at(&self, _text: &Self::Text, _start: usize) -> bool { false }
        fn find_at(&self, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> { None }
        fn read_captures_at(&self, _locs: &mut Locations, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> { None }
    }

    let ro = Arc::new(ExecReadOnly {
        res: vec!["b".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });

    let cache = RefCell::new(ProgramCacheInner::default());

    let exec = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };

    let result = exec.find_dfa_reverse_suffix(b"abc", 0);
    match result {
        dfa::Result::NoMatch(_) => {},
        _ => panic!("Expected NoMatch"),
    }
}

fn test_find_dfa_reverse_suffix_quit() {
    struct MyRegex;

    impl RegularExpression for MyRegex {
        type Text = [u8];

        fn slots_len(&self) -> usize { 0 }
        fn locations(&self) -> Locations { Locations::default() }
        fn next_after_empty(&self, _text: &Self::Text, _i: usize) -> usize { 0 }
        fn shortest_match_at(&self, _text: &Self::Text, _start: usize) -> Option<usize> { Some(1) }
        fn is_match_at(&self, _text: &Self::Text, _start: usize) -> bool { true }
        fn find_at(&self, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> { Some((0, 1)) }
        fn read_captures_at(&self, _locs: &mut Locations, _text: &Self::Text, _start: usize) -> Option<(usize, usize)> { Some((0, 1)) }
    }

    let ro = Arc::new(ExecReadOnly {
        res: vec!["a".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });

    let cache = RefCell::new(ProgramCacheInner::default());

    let exec = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };

    let result = exec.find_dfa_reverse_suffix(b"xyz", 0);
    match result {
        dfa::Result::Quit => {},
        _ => panic!("Expected Quit"),
    }
}

