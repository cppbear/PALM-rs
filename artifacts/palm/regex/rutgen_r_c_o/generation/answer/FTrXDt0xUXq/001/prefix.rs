// Answer 0

#[test]
fn test_byte_class_valid_bytes() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256],
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
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {
            compiled: HashMap::new(),
            trans: vec![],
            states: vec![],
            start_states: vec![],
            stack: vec![],
            flush_count: 0,
            size: 0,
        },
    };

    for byte_value in 0..=255 {
        let byte = Byte::byte(byte_value);
        let _ = fsm.byte_class(byte);
    }
}

#[test]
#[should_panic]
fn test_byte_class_eof() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256],
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
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {
            compiled: HashMap::new(),
            trans: vec![],
            states: vec![],
            start_states: vec![],
            stack: vec![],
            flush_count: 0,
            size: 0,
        },
    };
    
    let eof_byte = Byte::eof();
    let _ = fsm.byte_class(eof_byte);
}

