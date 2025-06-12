// Answer 0

#[test]
fn test_start_flags_reverse_at_text_end() {
    let text = b"hello";
    let fsm = Fsm {
        prog: &Program {
            insts: Vec::new(),
            matches: Vec::new(),
            captures: Vec::new(),
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: Vec::new(),
            only_utf8: false,
            is_bytes: false,
            is_dfa: false,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: false,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::default(),
            dfa_size_limit: 0,
        },
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {
            compiled: HashMap::default(),
            trans: Transitions::default(),
            states: Vec::new(),
            start_states: Vec::new(),
            stack: Vec::new(),
            flush_count: 0,
            size: 0,
        },
    };
    
    let (empty_flags, state_flags) = fsm.start_flags_reverse(text, text.len());
    
    assert!(empty_flags.start);
    assert!(empty_flags.end);
    assert!(empty_flags.start_line);
    assert!(empty_flags.end_line);
    assert!(!empty_flags.word_boundary);
    assert!(!empty_flags.not_word_boundary);
    assert!(!state_flags.is_word());
}

#[test]
fn test_start_flags_reverse_at_newline() {
    let text = b"hello\nworld";
    let fsm = Fsm {
        prog: &Program {
            insts: Vec::new(),
            matches: Vec::new(),
            captures: Vec::new(),
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: Vec::new(),
            only_utf8: false,
            is_bytes: false,
            is_dfa: false,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: false,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::default(),
            dfa_size_limit: 0,
        },
        start: STATE_START,
        at: 6,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {
            compiled: HashMap::default(),
            trans: Transitions::default(),
            states: Vec::new(),
            start_states: Vec::new(),
            stack: Vec::new(),
            flush_count: 0,
            size: 0,
        },
    };
    
    let (empty_flags, state_flags) = fsm.start_flags_reverse(text, fsm.at);
    
    assert!(empty_flags.start);
    assert!(empty_flags.end); // text isn't empty but at is at the end
    assert!(empty_flags.start_line);
    assert!(empty_flags.end_line);
    assert!(!empty_flags.word_boundary);
    assert!(!empty_flags.not_word_boundary);
    assert!(!state_flags.is_word());
}

#[test]
fn test_start_flags_reverse_middle_word() {
    let text = b"hello";
    let fsm = Fsm {
        prog: &Program {
            insts: Vec::new(),
            matches: Vec::new(),
            captures: Vec::new(),
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: Vec::new(),
            only_utf8: false,
            is_bytes: false,
            is_dfa: false,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: false,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::default(),
            dfa_size_limit: 0,
        },
        start: STATE_START,
        at: 1,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {
            compiled: HashMap::default(),
            trans: Transitions::default(),
            states: Vec::new(),
            start_states: Vec::new(),
            stack: Vec::new(),
            flush_count: 0,
            size: 0,
        },
    };
    
    let (empty_flags, state_flags) = fsm.start_flags_reverse(text, fsm.at);
    
    assert!(!empty_flags.start);
    assert!(!empty_flags.end);
    assert!(!empty_flags.start_line);
    assert!(!empty_flags.end_line);
    assert!(empty_flags.word_boundary);
    assert!(!empty_flags.not_word_boundary);
    assert!(!state_flags.is_word());
}

#[test]
fn test_start_flags_reverse_start_of_word() {
    let text = b"hello";
    let fsm = Fsm {
        prog: &Program {
            insts: Vec::new(),
            matches: Vec::new(),
            captures: Vec::new(),
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: Vec::new(),
            only_utf8: false,
            is_bytes: false,
            is_dfa: false,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: false,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::default(),
            dfa_size_limit: 0,
        },
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {
            compiled: HashMap::default(),
            trans: Transitions::default(),
            states: Vec::new(),
            start_states: Vec::new(),
            stack: Vec::new(),
            flush_count: 0,
            size: 0,
        },
    };
    
    let (empty_flags, state_flags) = fsm.start_flags_reverse(text, fsm.at);
    
    assert!(empty_flags.start);
    assert!(!empty_flags.end);
    assert!(empty_flags.start_line);
    assert!(!empty_flags.end_line);
    assert!(empty_flags.word_boundary);
    assert!(!empty_flags.not_word_boundary);
    assert!(state_flags.is_word());
}

