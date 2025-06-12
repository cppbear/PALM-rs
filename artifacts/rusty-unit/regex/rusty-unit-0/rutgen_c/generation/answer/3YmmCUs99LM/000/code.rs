// Answer 0

#[test]
fn test_forward_match() {
    struct TestInst;
    struct TestProgram {
        insts: Vec<TestInst>,
        matches: Vec<u32>,
        captures: Vec<Option<String>>,
        capture_name_idx: std::sync::Arc<std::collections::HashMap<String, usize>>,
        start: u32,
        byte_classes: Vec<u8>,
        only_utf8: bool,
        is_bytes: bool,
        is_dfa: bool,
        is_reverse: bool,
        is_anchored_start: bool,
        is_anchored_end: bool,
        has_unicode_word_boundary: bool,
        prefixes: (),
        dfa_size_limit: usize,
    }

    impl TestProgram {
        fn new() -> Self {
            TestProgram {
                insts: vec![],
                matches: vec![0],
                captures: vec![],
                capture_name_idx: std::sync::Arc::new(std::collections::HashMap::new()),
                start: 0,
                byte_classes: vec![],
                only_utf8: true,
                is_bytes: false,
                is_dfa: true,
                is_reverse: false,
                is_anchored_start: false,
                is_anchored_end: false,
                has_unicode_word_boundary: false,
                prefixes: (),
                dfa_size_limit: 512,
            }
        }
    }

    struct TestProgramCache {
        inner: CacheInner,
        qcur: SparseSet,
        qnext: SparseSet,
    }

    impl TestProgramCache {
        fn new() -> Self {
            TestProgramCache {
                inner: CacheInner {
                    compiled: std::collections::HashMap::new(),
                    trans: std::vec![],
                    states: vec![],
                    start_states: vec![STATE_UNKNOWN; 64],
                    stack: vec![],
                    flush_count: 0,
                    size: 0,
                },
                qcur: SparseSet { dense: vec![], sparse: vec![], size: 0},
                qnext: SparseSet { dense: vec![], sparse: vec![], size: 0},
            }
        }
    }

    let prog = TestProgram::new();
    let mut cache = TestProgramCache::new();
    let result = Fsm::forward(&prog, &cache, false, b"hello", 0);
    match result {
        Result::Match(_) => assert!(true),
        _ => assert!(false, "Should match"),
    }
}

#[test]
fn test_forward_no_match() {
    struct TestInst;
    struct TestProgram {
        insts: Vec<TestInst>,
        matches: Vec<u32>,
        captures: Vec<Option<String>>,
        capture_name_idx: std::sync::Arc<std::collections::HashMap<String, usize>>,
        start: u32,
        byte_classes: Vec<u8>,
        only_utf8: bool,
        is_bytes: bool,
        is_dfa: bool,
        is_reverse: bool,
        is_anchored_start: bool,
        is_anchored_end: bool,
        has_unicode_word_boundary: bool,
        prefixes: (),
        dfa_size_limit: usize,
    }

    impl TestProgram {
        fn new() -> Self {
            TestProgram {
                insts: vec![],
                matches: vec![0],
                captures: vec![],
                capture_name_idx: std::sync::Arc::new(std::collections::HashMap::new()),
                start: 0,
                byte_classes: vec![],
                only_utf8: true,
                is_bytes: false,
                is_dfa: true,
                is_reverse: false,
                is_anchored_start: false,
                is_anchored_end: false,
                has_unicode_word_boundary: false,
                prefixes: (),
                dfa_size_limit: 512,
            }
        }
    }

    struct TestProgramCache {
        inner: CacheInner,
        qcur: SparseSet,
        qnext: SparseSet,
    }

    impl TestProgramCache {
        fn new() -> Self {
            TestProgramCache {
                inner: CacheInner {
                    compiled: std::collections::HashMap::new(),
                    trans: std::vec![],
                    states: vec![],
                    start_states: vec![STATE_UNKNOWN; 64],
                    stack: vec![],
                    flush_count: 0,
                    size: 0,
                },
                qcur: SparseSet { dense: vec![], sparse: vec![], size: 0},
                qnext: SparseSet { dense: vec![], sparse: vec![], size: 0},
            }
        }
    }

    let prog = TestProgram::new();
    let mut cache = TestProgramCache::new();
    let result = Fsm::forward(&prog, &cache, false, b"world", 0);
    match result {
        Result::NoMatch(_) => assert!(true),
        _ => assert!(false, "Should not match"),
    }
}

#[test]
#[should_panic]
fn test_forward_quit() {
    struct TestInst;
    struct TestProgram {
        insts: Vec<TestInst>,
        matches: Vec<u32>,
        captures: Vec<Option<String>>,
        capture_name_idx: std::sync::Arc<std::collections::HashMap<String, usize>>,
        start: u32,
        byte_classes: Vec<u8>,
        only_utf8: bool,
        is_bytes: bool,
        is_dfa: bool,
        is_reverse: bool,
        is_anchored_start: bool,
        is_anchored_end: bool,
        has_unicode_word_boundary: bool,
        prefixes: (),
        dfa_size_limit: usize,
    }

    impl TestProgram {
        fn new() -> Self {
            TestProgram {
                insts: vec![],
                matches: vec![0],
                captures: vec![],
                capture_name_idx: std::sync::Arc::new(std::collections::HashMap::new()),
                start: 0,
                byte_classes: vec![],
                only_utf8: true,
                is_bytes: false,
                is_dfa: true,
                is_reverse: false,
                is_anchored_start: false,
                is_anchored_end: false,
                has_unicode_word_boundary: false,
                prefixes: (),
                dfa_size_limit: 512,
            }
        }
    }

    struct TestProgramCache {
        inner: CacheInner,
        qcur: SparseSet,
        qnext: SparseSet,
    }

    impl TestProgramCache {
        fn new() -> Self {
            TestProgramCache {
                inner: CacheInner {
                    compiled: std::collections::HashMap::new(),
                    trans: std::vec![],
                    states: vec![],
                    start_states: vec![STATE_UNKNOWN; 64],
                    stack: vec![],
                    flush_count: 0,
                    size: 0,
                },
                qcur: SparseSet { dense: vec![], sparse: vec![], size: 0},
                qnext: SparseSet { dense: vec![], sparse: vec![], size: 0},
            }
        }
    }

    let prog = TestProgram::new();
    let mut cache = TestProgramCache::new();
    let result = Fsm::forward(&prog, &cache, true, b"test", 0);
    match result {
        Result::Quit => panic!("Should not quit"),
        _ => assert!(true),
    }
}

