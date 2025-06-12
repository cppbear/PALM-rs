// Answer 0

#[test]
fn test_backtrack_with_empty_jobs() {
    struct TestInput {
        jobs: Vec<Job>,
    }

    impl Input for TestInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: 0, c: Char::from('\0'), byte: None, len: 0 }
        }
        
        fn next_char(&self, _at: InputAt) -> Char {
            Char::from('\0')
        }
        
        fn previous_char(&self, _at: InputAt) -> Char {
            Char::from('\0')
        }
        
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }
        
        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            None
        }
        
        fn len(&self) -> usize {
            0
        }
        
        fn is_empty(&self) -> bool {
            true
        }
        
        fn as_bytes(&self) -> &[u8] {
            &[]
        }
    }

    let mut matches: [bool; 1] = [false];
    let mut slots: [Option<usize>; 1] = [None];
    let dummy_program = Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 0 };
    let mut cache = Cache { jobs: vec![], visited: vec![] };

    let mut bounded = Bounded {
        prog: &dummy_program,
        input: TestInput { jobs: vec![] },
        matches: &mut matches,
        slots: &mut slots,
        m: &mut cache,
    };

    let start = InputAt { pos: 0, c: Char::from('\0'), byte: None, len: 0 };
    let result = bounded.backtrack(start);
    assert_eq!(result, false);
}

#[test]
fn test_backtrack_with_save_restore() {
    struct TestInput {
        jobs: Vec<Job>,
    }

    impl Input for TestInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: 0, c: Char::from('a'), byte: Some(b'a'), len: 1 }
        }
        
        fn next_char(&self, _at: InputAt) -> Char {
            Char::from('a')
        }
        
        fn previous_char(&self, _at: InputAt) -> Char {
            Char::from('a')
        }
        
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }
        
        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            None
        }
        
        fn len(&self) -> usize {
            1
        }
        
        fn is_empty(&self) -> bool {
            false
        }
        
        fn as_bytes(&self) -> &[u8] {
            &[b'a']
        }
    }

    let mut matches: [bool; 1] = [false];
    let mut slots: [Option<usize>; 2] = [None, None];
    let dummy_program = Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 0 };
    let mut cache = Cache { jobs: vec![], visited: vec![] };

    let mut bounded = Bounded {
        prog: &dummy_program,
        input: TestInput { jobs: vec![Job::SaveRestore { slot: 1, old_pos: Some(0) }] },
        matches: &mut matches,
        slots: &mut slots,
        m: &mut cache,
    };

    let start = InputAt { pos: 0, c: Char::from('a'), byte: Some(b'a'), len: 1 };
    let result = bounded.backtrack(start);
    assert_eq!(result, true);
}

#[test]
#[should_panic]
fn test_backtrack_with_invalid_slot_access() {
    struct TestInput {
        jobs: Vec<Job>,
    }

    impl Input for TestInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: 0, c: Char::from('b'), byte: Some(b'b'), len: 1 }
        }
        
        fn next_char(&self, _at: InputAt) -> Char {
            Char::from('b')
        }
        
        fn previous_char(&self, _at: InputAt) -> Char {
            Char::from('b')
        }
        
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }
        
        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            None
        }
        
        fn len(&self) -> usize {
            1
        }
        
        fn is_empty(&self) -> bool {
            false
        }
        
        fn as_bytes(&self) -> &[u8] {
            &[b'b']
        }
    }

    let mut matches: [bool; 1] = [false];
    let mut slots: [Option<usize>; 1] = [None];
    let dummy_program = Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 0 };
    let mut cache = Cache { jobs: vec![], visited: vec![] };

    let mut bounded = Bounded {
        prog: &dummy_program,
        input: TestInput { jobs: vec![Job::SaveRestore { slot: 1, old_pos: Some(0) }] },
        matches: &mut matches,
        slots: &mut slots,
        m: &mut cache,
    };

    let start = InputAt { pos: 0, c: Char::from('b'), byte: Some(b'b'), len: 1 };
    bounded.backtrack(start);
}

