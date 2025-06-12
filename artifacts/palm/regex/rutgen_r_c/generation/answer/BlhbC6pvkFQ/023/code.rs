// Answer 0

fn test_clear_cache_case1() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(256),
        states: vec![State { data: Box::new([0; 1]) }],
        start_states: vec![STATE_UNKNOWN, STATE_UNKNOWN, STATE_UNKNOWN],
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_DEAD,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let result = fsm.clear_cache();
    assert_eq!(result, true);
}

fn test_clear_cache_case2() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(256),
        states: vec![State { data: Box::new([0; 1]) }],
        start_states: vec![STATE_UNKNOWN, STATE_UNKNOWN, STATE_UNKNOWN],
        stack: vec![],
        flush_count: 4,
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
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 15,
        quit_after_match: false,
        last_match_si: STATE_MAX + 1,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let result = fsm.clear_cache();
    assert_eq!(result, true);
}

fn test_clear_cache_case3() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(256),
        states: vec![State { data: Box::new([0; 1]) }],
        start_states: vec![STATE_UNKNOWN],
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
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 20,
        quit_after_match: false,
        last_match_si: STATE_QUIT,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let result = fsm.clear_cache();
    assert_eq!(result, true);
}

