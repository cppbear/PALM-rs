// Answer 0

#[test]
fn test_add_state_success() {
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(256),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
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
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: true,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let state = State {
        data: Box::from([1, 2, 3]),
    };

    let state_ptr = fsm.add_state(state).unwrap();
    assert!(state_ptr != 0);
    assert_eq!(fsm.cache.states.len(), 1);
    assert_eq!(fsm.cache.size % 256, 0); // Ensure alignment with byte class size
}

#[test]
fn test_add_state_exceed_limit() {
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(1), // Force the condition to hit state limit
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
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
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: true,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let state = State {
        data: Box::from([1, 2, 3]),
    };

    let state_ptr = fsm.add_state(state);
    assert!(state_ptr.is_none());
    assert!(fsm.cache.states.is_empty());
}

