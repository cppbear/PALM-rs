// Answer 0

#[test]
fn test_add_state() {
    use std::sync::Arc;

    let mut state_cache = CacheInner {
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
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 10,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut state_cache,
    };

    let state_data = Box::new([1u8, 2, 3]);
    let state = State { data: state_data };

    let result = fsm.add_state(state);
    
    assert!(result.is_some());
    let si = result.unwrap();

    assert_eq!(fsm.cache.states.len(), 1);
    assert_eq!(fsm.cache.compiled.len(), 1);
    assert_eq!(fsm.cache.trans.num_states(), 1);
    assert_eq!(si, 0); // Since this is the first added state, the index should be 0.
}

