// Answer 0

#[test]
fn test_byte_class_valid_ascii() {
    let prog = Program {
        insts: Vec::new(),
        matches: Vec::new(),
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256], // assuming all bytes are in class 0 for simplicity
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {
            compiled: HashMap::new(),
            trans: Transitions::new(),
            states: vec![],
            start_states: vec![],
            stack: vec![],
            flush_count: 0,
            size: 0,
        },
    };

    for i in 0..=255u8 {
        let byte = Byte::byte(i);
        let result = fsm.byte_class(byte);
        assert_eq!(result, fsm.num_byte_classes() - 1); // As per our setup
    }
}

#[test]
fn test_byte_class_eof() {
    let prog = Program {
        insts: Vec::new(),
        matches: Vec::new(),
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256], // assuming all bytes are in class 0 for simplicity
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {
            compiled: HashMap::new(),
            trans: Transitions::new(),
            states: vec![],
            start_states: vec![],
            stack: vec![],
            flush_count: 0,
            size: 0,
        },
    };

    let eof_byte = Byte::eof();
    let result = fsm.byte_class(eof_byte);
    assert_eq!(result, fsm.num_byte_classes() - 1); // Edge case with EOF
}

