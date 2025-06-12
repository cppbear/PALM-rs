// Answer 0

#[test]
fn test_clear_cache_and_save_non_empty_cache_valid_state() {
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State { data: Box::from([0]) }],
        start_states: vec![STATE_UNKNOWN, STATE_DEAD],
        stack: vec![1],
        flush_count: 0,
        size: 10,
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
        dfa_size_limit: 1024,
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

    let mut state_ptr: StatePtr = 1;
    let result = fsm.clear_cache_and_save(Some(&mut state_ptr));
}

#[test]
fn test_clear_cache_and_save_cache_flush_limit() {
    let mut cache_inner = CacheInner {
        compiled: HashMap::from([(State { data: Box::from([1]) }, 1)]),
        trans: Transitions::new(),
        states: vec![State { data: Box::from([1]) }, State { data: Box::from([2]) }],
        start_states: vec![STATE_UNKNOWN, STATE_DEAD],
        stack: vec![1],
        flush_count: 2,
        size: 10,
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
        dfa_size_limit: 1024,
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

    let mut state_ptr: StatePtr = 1;
    let result = fsm.clear_cache_and_save(Some(&mut state_ptr));
}

#[test]
fn test_clear_cache_and_save_valid_state_and_cache() {
    let mut cache_inner = CacheInner {
        compiled: HashMap::from([(State { data: Box::from([0, 1]) }, 2)]),
        trans: Transitions::new(),
        states: vec![State { data: Box::from([3]) }],
        start_states: vec![STATE_UNKNOWN],
        stack: vec![1],
        flush_count: 2,
        size: 15,
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
        dfa_size_limit: 2048,
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

    let mut state_ptr: StatePtr = 2;
    let result = fsm.clear_cache_and_save(Some(&mut state_ptr));
}

