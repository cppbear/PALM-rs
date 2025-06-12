// Answer 0

#[test]
fn test_add_state_success() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions {
            table: vec![],
            num_byte_classes: 256,
        },
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let program = Program {
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
        has_unicode_word_boundary: true,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };

    let state = State {
        data: Box::new([1, 2, 3]),
    };

    let mut fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let result = fsm.add_state(state);
    assert!(result.is_some());
    assert_eq!(fsm.cache.states.len(), 1);
    assert_eq!(fsm.cache.compiled.len(), 1);
}

#[test]
#[should_panic]
fn test_add_state_failure_limit_reached() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions {
            table: vec![0; (STATE_MAX as usize + 1)], // Filling to max to force limit
            num_byte_classes: 256,
        },
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let program = Program {
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
        has_unicode_word_boundary: true,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };

    let state = State {
        data: Box::new([1, 2, 3]),
    };

    let mut fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    // This should lead to panic since the state limit is reached
    let _ = fsm.add_state(state);
}

#[test]
fn test_add_state_no_unicode_beyond_ascii() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions {
            table: vec![],
            num_byte_classes: 256,
        },
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let program = Program {
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
        has_unicode_word_boundary: true,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };

    let state1 = State {
        data: Box::new([4, 5, 6]),
    };

    let state2 = State {
        data: Box::new([7, 8, 9]),
    };

    let mut fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let result1 = fsm.add_state(state1);
    assert!(result1.is_some());

    let result2 = fsm.add_state(state2);
    assert!(result2.is_some());
    
    assert_eq!(fsm.cache.states.len(), 2);
    assert_eq!(fsm.cache.compiled.len(), 2);
}

