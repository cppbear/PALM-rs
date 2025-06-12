// Answer 0

#[test]
fn test_clear_with_non_empty_cache() {
    #[derive(Debug)]
    struct TestInput;

    impl Input for TestInput {
        fn at(&self, _: usize) -> InputAt {
            InputAt::default()
        }
        fn next_char(&self, _: InputAt) -> Char {
            Char::default()
        }
        fn previous_char(&self, _: InputAt) -> Char {
            Char::default()
        }
        fn is_empty_match(&self, _: InputAt, _: &InstEmptyLook) -> bool {
            false
        }
        fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> {
            None
        }
        fn len(&self) -> usize {
            5
        }
        fn is_empty(&self) -> bool {
            false
        }
        fn as_bytes(&self) -> &[u8] {
            &[]
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let mut cache = Cache {
        jobs: vec![Job::Inst { ip: 0, at: InputAt::default() }],
        visited: vec![1],
    };

    let mut visited = vec![1];
    let mut slots = vec![];

    let mut bounded = Bounded {
        prog: &program,
        input: TestInput,
        matches: &mut slots,
        slots: &mut slots,
        m: &mut cache,
    };

    bounded.clear();
    
    assert_eq!(cache.jobs.len(), 0);
    assert_eq!(cache.visited, vec![0]);
}

#[test]
fn test_clear_with_empty_cache_truncate() {
    #[derive(Debug)]
    struct TestInput;

    impl Input for TestInput {
        fn at(&self, _: usize) -> InputAt {
            InputAt::default()
        }
        fn next_char(&self, _: InputAt) -> Char {
            Char::default()
        }
        fn previous_char(&self, _: InputAt) -> Char {
            Char::default()
        }
        fn is_empty_match(&self, _: InputAt, _: &InstEmptyLook) -> bool {
            false
        }
        fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> {
            None
        }
        fn len(&self) -> usize {
            3
        }
        fn is_empty(&self) -> bool {
            false
        }
        fn as_bytes(&self) -> &[u8] {
            &[]
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let mut cache = Cache {
        jobs: vec![],
        visited: vec![1, 1],
    };

    let mut visited = vec![1, 1];
    let mut slots = vec![];

    let mut bounded = Bounded {
        prog: &program,
        input: TestInput,
        matches: &mut slots,
        slots: &mut slots,
        m: &mut cache,
    };

    bounded.clear();

    assert_eq!(cache.jobs.len(), 0);
    assert_eq!(cache.visited, vec![0]);
}

#[test]
fn test_clear_with_increased_capacity() {
    #[derive(Debug)]
    struct TestInput;

    impl Input for TestInput {
        fn at(&self, _: usize) -> InputAt {
            InputAt::default()
        }
        fn next_char(&self, _: InputAt) -> Char {
            Char::default()
        }
        fn previous_char(&self, _: InputAt) -> Char {
            Char::default()
        }
        fn is_empty_match(&self, _: InputAt, _: &InstEmptyLook) -> bool {
            false
        }
        fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> {
            None
        }
        fn len(&self) -> usize {
            10
        }
        fn is_empty(&self) -> bool {
            false
        }
        fn as_bytes(&self) -> &[u8] {
            &[]
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let mut cache = Cache {
        jobs: vec![],
        visited: vec![1; 5],
    };

    let mut visited = vec![1; 5];
    let mut slots = vec![];

    let mut bounded = Bounded {
        prog: &program,
        input: TestInput,
        matches: &mut slots,
        slots: &mut slots,
        m: &mut cache,
    };

    bounded.clear();

    assert_eq!(cache.visited.len(), 10);
    assert_eq!(cache.visited[0..5], [0, 0, 0, 0, 0]);
}

