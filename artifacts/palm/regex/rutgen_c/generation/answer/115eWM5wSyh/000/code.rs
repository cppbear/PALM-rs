// Answer 0

#[test]
fn test_next_state_with_valid_state() {
    struct TestCache {
        trans: Transitions,
    }

    impl TestCache {
        fn new() -> Self {
            TestCache {
                trans: Transitions {
                    table: vec![STATE_UNKNOWN, STATE_DEAD, STATE_QUIT],
                    num_byte_classes: 1,
                },
            }
        }
    }

    let mut sparse_cur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    
    let mut sparse_next = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    
    let byte = Byte(0);
    let mut cache = TestCache::new();
    
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 100,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let state = 0; // valid state pointer
    let result = fsm.next_state(&mut sparse_cur, &mut sparse_next, state, byte);
    assert!(result.is_some());
}

#[test]
fn test_next_state_with_dead_state() {
    let mut sparse_cur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let mut sparse_next = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    
    let byte = Byte(0);
    let mut cache = TestCache::new();
    
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 100,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let state = STATE_DEAD; // dead state
    let result = fsm.next_state(&mut sparse_cur, &mut sparse_next, state, byte);
    assert_eq!(result, Some(STATE_DEAD));
}

#[test]
fn test_next_state_with_quit() {
    struct TestCache {
        trans: Transitions,
    }

    impl TestCache {
        fn new() -> Self {
            let mut trans = Transitions {
                table: vec![STATE_UNKNOWN, STATE_QUIT],
                num_byte_classes: 1,
            };
            trans.set_next(0, 0, STATE_QUIT); // Set for testing quit
            TestCache { trans }
        }
    }

    let mut sparse_cur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let mut sparse_next = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let byte = Byte(0);
    let mut cache = TestCache::new();

    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 100,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let state = 0; // valid state pointer
    let result = fsm.next_state(&mut sparse_cur, &mut sparse_next, state, byte);
    assert_eq!(result, None);
}

