// Answer 0

#[test]
fn test_clear_cache_normal_case() {
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(256),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
        flush_count: 3,
        size: 0,
    };
    
    let state_data = Box::new([1u8; 10]);
    let last_match_data = Box::new([2u8; 10]);
    
    let start_state = State { data: state_data };
    let last_match_state = State { data: last_match_data };
    
    cache_inner.states.push(start_state);
    cache_inner.states.push(last_match_state);
    
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };
    
    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 20,
        quit_after_match: false,
        last_match_si: STATE_MAX + 1,
        last_cache_flush: 20,
        cache: &mut cache_inner,
    };

    fsm.clear_cache();
}

#[test]
fn test_clear_cache_edge_case_zero_start_states() {
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(256),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
        flush_count: 3,
        size: 0,
    };
    
    let state_data = Box::new([1u8; 10]);
    let last_match_data = Box::new([2u8; 10]);
    
    let start_state = State { data: state_data };
    let last_match_state = State { data: last_match_data };
    
    cache_inner.states.push(start_state);
    cache_inner.states.push(last_match_state);
    
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };
    
    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 20,
        quit_after_match: false,
        last_match_si: STATE_MAX + 1,
        last_cache_flush: 20,
        cache: &mut cache_inner,
    };

    fsm.clear_cache();
}

