// Answer 0

#[test]
fn test_clear_cache_and_save_no_current_state() {
    let states: Vec<State> = Vec::new();
    let cache = CacheInner {
        compiled: HashMap::new(),
        trans: vec![],
        states,
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
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
        dfa_size_limit: 256,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    assert!(fsm.clear_cache_and_save(None));
}

#[test]
fn test_clear_cache_and_save_with_current_state() {
    let state = State { data: Box::new([0u8; 256]) };
    let mut states: Vec<State> = vec![state.clone()];
    
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: vec![],
        states,
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
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
        dfa_size_limit: 256,
    };

    let mut current_state: StatePtr = 0; // existing state index
    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    assert!(fsm.clear_cache_and_save(Some(&mut current_state)));
    assert_eq!(current_state, 0);  // Ensure that the state has been restored to the same index
}

