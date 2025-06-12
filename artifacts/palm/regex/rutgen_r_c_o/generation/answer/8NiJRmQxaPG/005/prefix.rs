// Answer 0

#[test]
fn test_cached_state_dead_key_none() {
    let program = Program {
        insts: Vec::new(),
        matches: Vec::new(),
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };

    let cache = &mut CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
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
        cache,
    };

    let sparse_set = SparseSet {
        dense: Vec::new(),
        sparse: Vec::new(),
        size: 0,
    };

    let state_flags = StateFlags(0);
    let current_state: Option<&mut StatePtr> = None;

    fsm.cached_state(&sparse_set, state_flags, current_state);
}

#[test]
fn test_cached_state_dead_key_none_with_state() {
    let program = Program {
        insts: Vec::new(),
        matches: Vec::new(),
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };

    let cache = &mut CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
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
        cache,
    };

    let sparse_set = SparseSet {
        dense: Vec::new(),
        sparse: Vec::new(),
        size: 0,
    };

    let state_flags = StateFlags(0);
    let mut current_state: StatePtr = STATE_UNKNOWN;
    let current_state_opt: Option<&mut StatePtr> = Some(&mut current_state);

    fsm.cached_state(&sparse_set, state_flags, current_state_opt);
}

