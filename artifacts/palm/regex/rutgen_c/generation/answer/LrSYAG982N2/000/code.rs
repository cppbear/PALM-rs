// Answer 0

#[test]
fn test_prefix_at_with_no_prefixes() {
    struct TestProgram {
        prefixes: LiteralSearcher,
    }

    impl TestProgram {
        fn new() -> Self {
            TestProgram {
                prefixes: LiteralSearcher::empty(),
            }
        }
    }

    let program = TestProgram::new();
    let fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {
            compiled: HashMap::new(),
            trans: vec![],
            states: vec![],
            start_states: vec![],
            stack: vec![],
            flush_count: 0,
            size: 0,
        },
    };

    assert_eq!(fsm.prefix_at(b"some text", 0), Some(0));
}

#[test]
fn test_prefix_at_with_valid_prefix() {
    struct TestProgram {
        prefixes: LiteralSearcher,
    }

    impl TestProgram {
        fn new() -> Self {
            let literals = Literals::from(vec![b"some".to_vec()]);
            TestProgram {
                prefixes: LiteralSearcher::prefixes(literals),
            }
        }
    }

    let program = TestProgram::new();
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: vec![],
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    assert_eq!(fsm.prefix_at(b"some other text", 0), Some(0));
}

#[test]
fn test_prefix_at_with_no_match() {
    struct TestProgram {
        prefixes: LiteralSearcher,
    }

    impl TestProgram {
        fn new() -> Self {
            let literals = Literals::from(vec![b"some".to_vec()]);
            TestProgram {
                prefixes: LiteralSearcher::prefixes(literals),
            }
        }
    }

    let program = TestProgram::new();
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: vec![],
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    assert_eq!(fsm.prefix_at(b"other text", 0), None);
}

