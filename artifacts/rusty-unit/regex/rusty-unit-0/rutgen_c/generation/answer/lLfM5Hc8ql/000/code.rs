// Answer 0

#[test]
fn test_clear_empty_cache() {
    struct MockInput {
        len: usize,
    }
    
    impl Input for MockInput {
        fn at(&self, i: usize) -> InputAt { InputAt::new(i) }
        fn next_char(&self, _: InputAt) -> Char { 'a' }
        fn previous_char(&self, _: InputAt) -> Char { 'a' }
        fn is_empty_match(&self, _: InputAt, _: &InstEmptyLook) -> bool { false }
        fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> { None }
        fn len(&self) -> usize { self.len }
        fn is_empty(&self) -> bool { self.len == 0 }
        fn as_bytes(&self) -> &[u8] { &[] }
    }

    let mut prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut cache = Cache { clist: Threads::new(), nlist: Threads::new(), stack: vec![] };
    let mut matches = vec![false; 10];
    let mut slots = vec![Slot::default(); 1];

    let mut bounded = Bounded {
        prog: &prog,
        input: MockInput { len: 10 },
        matches: &mut matches,
        slots: &mut slots,
        m: &mut cache,
    };

    bounded.clear();
    assert_eq!(cache.jobs.len(), 0);
    assert_eq!(cache.visited.len(), 1);
    assert_eq!(cache.visited[0], 0);
}

#[test]
fn test_clear_populated_cache() {
    struct MockInput {
        len: usize,
    }

    impl Input for MockInput {
        fn at(&self, i: usize) -> InputAt { InputAt::new(i) }
        fn next_char(&self, _: InputAt) -> Char { 'a' }
        fn previous_char(&self, _: InputAt) -> Char { 'a' }
        fn is_empty_match(&self, _: InputAt, _: &InstEmptyLook) -> bool { false }
        fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> { None }
        fn len(&self) -> usize { self.len }
        fn is_empty(&self) -> bool { self.len == 0 }
        fn as_bytes(&self) -> &[u8] { &[] }
    }

    let mut prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut cache = Cache { jobs: vec![Job::Inst { ip: InstPtr::default(), at: InputAt::new(0) }], visited: vec![1] };
    let mut matches = vec![false; 10];
    let mut slots = vec![Slot::default(); 1];

    let mut bounded = Bounded {
        prog: &prog,
        input: MockInput { len: 10 },
        matches: &mut matches,
        slots: &mut slots,
        m: &mut cache,
    };

    bounded.clear();
    assert_eq!(cache.jobs.len(), 0);
    assert_eq!(cache.visited.len(), 1);
    assert_eq!(cache.visited[0], 0); // Ensure the cache was cleared
}

