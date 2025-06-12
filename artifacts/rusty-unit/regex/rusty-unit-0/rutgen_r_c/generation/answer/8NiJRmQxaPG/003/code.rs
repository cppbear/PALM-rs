// Answer 0

#[test]
fn test_cached_state_with_existing_key() {
    let mut cached_states: HashMap<State, StatePtr> = HashMap::new();

    // Create a sample program with hypothetical instructions
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
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 10,
    };

    let mut cache_inner = CacheInner {
        compiled: cached_states,
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    // Create a hypothetical SparseSet and StateFlags
    let sparse_set = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let mut state_flags = StateFlags(0);
    let mut state_ptr = STATE_START;

    // Manually populate the cache with a key for testing
    let state = State {
        data: vec![0].into_boxed_slice(),
    };
    fsm.cache.compiled.insert(state.clone(), STATE_START);

    // Update cache size to be at limit 
    fsm.cache.size = fsm.prog.dfa_size_limit;

    // Call the method
    let result = fsm.cached_state(&sparse_set, state_flags, Some(&mut state_ptr));
    
    // Check if the result is a valid StatePtr
    assert_eq!(result, Some(STATE_START));
}

#[test]
fn test_cached_state_without_available_key() {
    let mut cached_states: HashMap<State, StatePtr> = HashMap::new();

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
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 10,
    };

    let mut cache_inner = CacheInner {
        compiled: cached_states,
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let sparse_set = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let mut state_flags = StateFlags(0);
    let current_state = STATE_START;

    // Call the method with a SparseSet that will not match any key
    let result = fsm.cached_state(&sparse_set, state_flags, Some(&mut current_state));
    
    // Result should be STATE_DEAD, which means no valid state can be cached
    assert_eq!(result, Some(STATE_DEAD));
}

