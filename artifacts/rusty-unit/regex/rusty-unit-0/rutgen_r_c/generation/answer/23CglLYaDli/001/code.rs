// Answer 0

#[test]
fn test_u8_class_lower_bound() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };
    
    let fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {
            compiled: HashMap::new(),
            trans: Transitions::default(),
            states: vec![],
            start_states: vec![],
            stack: vec![],
            flush_count: 0,
            size: 0,
        },
    };

    assert_eq!(fsm.u8_class(0), 0);
}

#[test]
fn test_u8_class_upper_bound() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };
    
    let fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {
            compiled: HashMap::new(),
            trans: Transitions::default(),
            states: vec![],
            start_states: vec![],
            stack: vec![],
            flush_count: 0,
            size: 0,
        },
    };

    assert_eq!(fsm.u8_class(255), 0);
}

#[should_panic]
#[test]
fn test_u8_class_panic() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![], // Empty byte_classes to trigger panic
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };

    let fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {
            compiled: HashMap::new(),
            trans: Transitions::default(),
            states: vec![],
            start_states: vec![],
            stack: vec![],
            flush_count: 0,
            size: 0,
        },
    };

    let _ = fsm.u8_class(0); // Accessing index 0 should panic
}

