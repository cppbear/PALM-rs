// Answer 0

#[test]
fn test_cached_state_valid_key_and_cache() {
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
        flush_count: 0,
        size: 0,
    };

    let prog = Program {
        insts: Vec::new(),
        matches: vec![0],
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };

    let mut state_flags = StateFlags(10);
    let mut sparse_set = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let state_ptr: StatePtr = 5;
    cache_inner.compiled.insert(State { data: Box::new([]) }, state_ptr);
    
    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let mut current_state = Some(&mut state_ptr);
    let _ = fsm.cached_state(&sparse_set, state_flags, current_state);
}

#[test]
fn test_cached_state_exceed_cache_limit() {
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
        flush_count: 0,
        size: 20, // set size to exceed limit
    };

    let prog = Program {
        insts: Vec::new(),
        matches: vec![0],
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };

    let mut state_flags = StateFlags(10);
    let mut sparse_set = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let state_ptr: StatePtr = 5;
    cache_inner.compiled.insert(State { data: Box::new([]) }, state_ptr);

    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let mut current_state = Some(&mut state_ptr);
    let _ = fsm.cached_state(&sparse_set, state_flags, current_state);
}

#[test]
fn test_cached_state_key_not_found() {
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
        flush_count: 0,
        size: 0,
    };

    let prog = Program {
        insts: Vec::new(),
        matches: vec![0],
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };

    let mut state_flags = StateFlags(0); // state flags that do not yield valid key
    let mut sparse_set = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let current_state: Option<&mut StatePtr> = None;
    let _ = fsm.cached_state(&sparse_set, state_flags, current_state);
}

