// Answer 0

#[test]
fn test_clear_cache_success() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(256),
        states: vec![State { data: Box::new([0; 256]) }],
        start_states: vec![],
        stack: vec![0; 10],
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
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 1,
        at: 5,
        quit_after_match: false,
        last_match_si: STATE_MATCH + 1,
        last_cache_flush: 10,
        cache: &mut cache,
    };

    fsm.clear_cache();
}

#[test]
fn test_clear_cache_success_with_different_last_cache_flush() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(256),
        states: vec![State { data: Box::new([0; 256]) }],
        start_states: vec![],
        stack: vec![0; 10],
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
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 1,
        at: 15,
        quit_after_match: false,
        last_match_si: STATE_MATCH + 1,
        last_cache_flush: 10,
        cache: &mut cache,
    };

    fsm.clear_cache();
}

