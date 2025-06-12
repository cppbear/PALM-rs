// Answer 0

#[test]
fn test_exec_dfa_reverse_suffix_valid_case() {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::sync::Arc;

    struct TestRegularExpression;

    impl RegularExpression for TestRegularExpression {
        type Text = [u8];
        fn slots_len(&self) -> usize { 0 }
        fn locations(&self) -> Locations { unimplemented!() }
        fn next_after_empty(&self, _: &Self::Text, _: usize) -> usize { 0 }
        fn shortest_match_at(&self, _: &Self::Text, _: usize) -> Option<usize> { None }
        fn is_match_at(&self, _: &Self::Text, _: usize) -> bool { false }
        fn find_at(&self, _: &Self::Text, _: usize) -> Option<(usize, usize)> { None }
        fn read_captures_at(&self, _: &mut Locations, _: &Self::Text, _: usize) -> Option<(usize, usize)> { None }
    }

    let suffix_packed = FreqyPacked {
        pat: vec![b'a'], // lcs.len() == 1
        char_len: 1,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 0,
    };

    let literal_searcher = LiteralSearcher {
        complete: true,
        lcp: suffix_packed.clone(),
        lcs: suffix_packed,
        matcher: Matcher::Empty,
    };

    let program_cache = ProgramCache::default();
    let program = Program {
        insts: Vec::new(),
        matches: Vec::new(),
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: literal_searcher.clone(),
        dfa_size_limit: 1024,
    };

    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![String::from("a")],
            nfa: program.clone(),
            dfa: program.clone(),
            dfa_reverse: program,
            suffixes: literal_searcher,
            match_type: MatchType::Regular,
        }),
        cache: &program_cache,
    };

    let text = b"abcdeaa"; // end == text.len()
    let original_start = 0;

    let result = exec_no_sync.exec_dfa_reverse_suffix(text, original_start);

    match result {
        Some(dfa::Result::Match((start, end))) => {
            assert!(start >= 0);
            assert_eq!(end, text.len());
        },
        _ => panic!("Expected a match result"),
    }
}

#[test]
fn test_exec_dfa_reverse_suffix_no_match() {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::sync::Arc;

    struct TestRegularExpression;

    impl RegularExpression for TestRegularExpression {
        type Text = [u8];
        fn slots_len(&self) -> usize { 0 }
        fn locations(&self) -> Locations { unimplemented!() }
        fn next_after_empty(&self, _: &Self::Text, _: usize) -> usize { 0 }
        fn shortest_match_at(&self, _: &Self::Text, _: usize) -> Option<usize> { None }
        fn is_match_at(&self, _: &Self::Text, _: usize) -> bool { false }
        fn find_at(&self, _: &Self::Text, _: usize) -> Option<(usize, usize)> { None }
        fn read_captures_at(&self, _: &mut Locations, _: &Self::Text, _: usize) -> Option<(usize, usize)> { None }
    }

    let suffix_packed = FreqyPacked {
        pat: vec![b'a'], // lcs.len() == 1
        char_len: 1,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 0,
    };

    let literal_searcher = LiteralSearcher {
        complete: true,
        lcp: suffix_packed.clone(),
        lcs: suffix_packed,
        matcher: Matcher::Empty,
    };

    let program_cache = ProgramCache::default();
    let program = Program {
        insts: Vec::new(),
        matches: Vec::new(),
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: literal_searcher.clone(),
        dfa_size_limit: 1024,
    };

    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![String::from("a")],
            nfa: program.clone(),
            dfa: program.clone(),
            dfa_reverse: program,
            suffixes: literal_searcher,
            match_type: MatchType::Regular,
        }),
        cache: &program_cache,
    };

    let text = b"bcde"; // No match found
    let original_start = 0;

    let result = exec_no_sync.exec_dfa_reverse_suffix(text, original_start);

    match result {
        Some(dfa::Result::NoMatch(_)) => {
            assert!(true);
        },
        _ => panic!("Expected no match result"),
    }
}

