// Answer 0

#[test]
fn test_clear_cache_basic() {
    struct TestCacheInner {
        compiled: HashMap<State, StatePtr>,
        trans: Transitions,
        states: Vec<State>,
        start_states: Vec<StatePtr>,
        flush_count: u64,
        size: usize,
    }

    struct TestFsm<'a> {
        prog: &'a Program,
        start: StatePtr,
        at: usize,
        last_cache_flush: usize,
        cache: &'a mut TestCacheInner,
        last_match_si: StatePtr,
    }

    let mut states = Vec::new();
    let state_data: State = State { data: Box::new([1]) };
    states.push(state_data.clone());
    let mut cache_inner = TestCacheInner {
        compiled: HashMap::new(),
        trans: Transitions { table: Vec::new(), num_byte_classes: 256 },
        states,
        start_states: vec![STATE_UNKNOWN],
        flush_count: 0,
        size: 0,
    };
    
    let program = Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 };
    
    let mut fsm = TestFsm {
        prog: &program,
        start: STATE_UNKNOWN,
        at: 0,
        last_cache_flush: 0,
        cache: &mut cache_inner,
        last_match_si: STATE_UNKNOWN,
    };

    assert!(fsm.clear_cache());
}

#[test]
fn test_clear_cache_no_modify_if_too_slow() {
    struct TestCacheInner {
        compiled: HashMap<State, StatePtr>,
        trans: Transitions,
        states: Vec<State>,
        start_states: Vec<StatePtr>,
        flush_count: u64,
        size: usize,
    }

    struct TestFsm<'a> {
        prog: &'a Program,
        start: StatePtr,
        at: usize,
        last_cache_flush: usize,
        cache: &'a mut TestCacheInner,
        last_match_si: StatePtr,
    }

    let mut states = Vec::new();
    let state_data: State = State { data: Box::new([1]) };
    states.push(state_data.clone());
    let mut cache_inner = TestCacheInner {
        compiled: HashMap::new(),
        trans: Transitions { table: Vec::new(), num_byte_classes: 256 },
        states,
        start_states: vec![STATE_UNKNOWN],
        flush_count: 3, // At 3 flush counts, should check for being too slow
        size: 0,
    };
    
    let program = Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 };
    
    let mut fsm = TestFsm {
        prog: &program,
        start: STATE_UNKNOWN,
        at: 20, // Simulate moving slowly
        last_cache_flush: 10,
        cache: &mut cache_inner,
        last_match_si: STATE_UNKNOWN,
    };

    assert!(!fsm.clear_cache());
}

#[test]
fn test_clear_cache_flushing() {
    struct TestCacheInner {
        compiled: HashMap<State, StatePtr>,
        trans: Transitions,
        states: Vec<State>,
        start_states: Vec<StatePtr>,
        flush_count: u64,
        size: usize,
    }

    struct TestFsm<'a> {
        prog: &'a Program,
        start: StatePtr,
        at: usize,
        last_cache_flush: usize,
        cache: &'a mut TestCacheInner,
        last_match_si: StatePtr,
    }

    let mut states = Vec::new();
    let state_data: State = State { data: Box::new([1]) };
    states.push(state_data.clone());
    let mut cache_inner = TestCacheInner {
        compiled: HashMap::new(),
        trans: Transitions { table: Vec::new(), num_byte_classes: 256 },
        states,
        start_states: vec![STATE_UNKNOWN],
        flush_count: 2, // Less than 3, so should flush
        size: 0,
    };
    
    let program = Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 };
    
    let mut fsm = TestFsm {
        prog: &program,
        start: STATE_UNKNOWN,
        at: 0,
        last_cache_flush: 0,
        cache: &mut cache_inner,
        last_match_si: STATE_UNKNOWN,
    };

    assert!(fsm.clear_cache());
}

