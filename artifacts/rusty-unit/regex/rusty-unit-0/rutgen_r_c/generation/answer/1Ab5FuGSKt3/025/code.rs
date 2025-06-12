// Answer 0

#[test]
fn test_exec_with_prefixes_and_non_matched() {
    struct TestInput;
    
    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: Char::from(0), byte: None, len: 1 }
        }
        fn next_char(&self, at: InputAt) -> Char {
            Char::from(0)
        }
        fn previous_char(&self, at: InputAt) -> Char {
            Char::from(0)
        }
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }
        fn prefix_at(&self, _prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> {
            Some(at)
        }
        fn len(&self) -> usize {
            10
        }
        fn is_empty(&self) -> bool {
            false
        }
        fn as_bytes(&self) -> &[u8] {
            b"test_input"
        }
    }

    let program = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 0,
    };

    let mut cache = Cache {
        jobs: vec![],
        visited: vec![],
    };

    let mut matches = vec![false; 10];
    let mut slots = vec![Slot::default(); 10];

    let mut bounded = Bounded {
        prog: &program,
        input: TestInput,
        matches: &mut matches,
        slots: &mut slots,
        m: &mut cache,
    };

    let result = bounded.exec_(
        InputAt { pos: 0, c: Char::from(0), byte: None, len: 0 }
    );

    assert_eq!(result, true);
}

