// Answer 0

#[test]
fn test_clear_cache_and_save_empty_cache() {
    let mut states: Vec<State> = Vec::new();
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
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
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

    let current_state: Option<&mut StatePtr> = None;

    let result = fsm.clear_cache_and_save(current_state);
    
    assert_eq!(result, true);
}

#[test]
fn test_clear_cache_and_save_with_current_state() {
    let mut states: Vec<State> = Vec::new();
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
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
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

    let state_ptr: StatePtr = 0;
    let mut current_state: Option<&mut StatePtr> = Some(&mut state_ptr);

    let result = fsm.clear_cache_and_save(current_state);
    
    assert_eq!(result, true);
}

