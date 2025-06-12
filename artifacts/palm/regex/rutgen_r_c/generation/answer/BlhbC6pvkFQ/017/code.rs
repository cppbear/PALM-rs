// Answer 0

fn test_clear_cache_return_true() {
    struct MockSparseSet;
    struct MockProgramCache;
    
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(256),
        states: vec![],
        start_states: vec![STATE_UNKNOWN; 1],
        stack: vec![],
        flush_count: 3,
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
        dfa_size_limit: 1024,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_DEAD,
        last_cache_flush: 10,
        cache: &mut cache,
    };

    assert_eq!(fsm.clear_cache(), true);
}

fn test_clear_cache_does_not_clear_on_slow_processing() {
    struct MockSparseSet;
    struct MockProgramCache;

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(256),
        states: vec![State { data: vec![0; 10].into_boxed_slice() }],
        start_states: vec![STATE_UNKNOWN; 1],
        stack: vec![],
        flush_count: 3,
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
        dfa_size_limit: 1024,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 20,
        quit_after_match: false,
        last_match_si: STATE_MAX + 1,
        last_cache_flush: 10,
        cache: &mut cache,
    };

    assert_eq!(fsm.clear_cache(), false);
}

