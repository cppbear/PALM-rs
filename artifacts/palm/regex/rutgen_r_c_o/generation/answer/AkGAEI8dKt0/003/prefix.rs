// Answer 0

#[test]
fn test_add_state_with_valid_data_and_unicode_boundary() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(128),
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
        has_unicode_word_boundary: true,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1000,
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
    
    let state_data = vec![0u8; STATE_MAX as usize / 2];
    let state = State {
        data: state_data.into_boxed_slice(),
    };
    
    fsm.add_state(state);
}

#[test]
fn test_add_state_with_unicode_boundary_and_exceed_max_states() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(1),
        states: vec![State {
            data: vec![0u8; 10].into_boxed_slice(),
        }],
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
        has_unicode_word_boundary: true,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1000,
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

    let state = State {
        data: vec![0u8; STATE_MAX as usize].into_boxed_slice(),
    };

    let result = fsm.add_state(state);
    assert!(result.is_none());
}

#[test]
fn test_add_state_with_unicode_boundary_and_non_ascii_b() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(128),
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
        has_unicode_word_boundary: true,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1000,
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
    
    let state = State {
        data: vec![0u8; 10].into_boxed_slice(),
    };

    let state_ptr = fsm.add_state(state);

    for b in 128..256 {
        let cls = fsm.cache.trans.byte_class(Byte::byte(b as u8));
        assert_eq!(fsm.cache.trans.next(state_ptr.unwrap(), cls), STATE_QUIT);
    }
}

#[test]
fn test_add_state_without_unicode_boundary() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(128),
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1000,
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

    let state = State {
        data: vec![0u8; 10].into_boxed_slice(),
    };

    let _ = fsm.add_state(state);
    assert_eq!(fsm.cache.trans.num_states(), 1);
}

