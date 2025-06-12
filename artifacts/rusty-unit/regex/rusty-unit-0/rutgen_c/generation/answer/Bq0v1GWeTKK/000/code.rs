// Answer 0

#[test]
fn test_start_ptr_with_prefix() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: std::sync::Arc::new(std::collections::HashMap::new()),
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
        dfa_size_limit: 1024,
    };

    let mut fsm = Fsm {
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

    prog.prefixes = LiteralSearcher::from("prefix");

    let state_ptr = 0;
    assert_eq!(fsm.start_ptr(state_ptr), state_ptr | STATE_START);
}

#[test]
fn test_start_ptr_without_prefix() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: std::sync::Arc::new(std::collections::HashMap::new()),
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
        dfa_size_limit: 1024,
    };

    let mut fsm = Fsm {
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

    let state_ptr = 0;
    assert_eq!(fsm.start_ptr(state_ptr), state_ptr);
}

