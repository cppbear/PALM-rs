// Answer 0

#[test]
fn test_start_flags_at_non_zero_non_empty_text() {
    let text = b"Hello, world!";
    let at = 1; // at > 0, at < text.len()
    
    let fsm = Fsm {
        prog: &Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: true, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024},
        start: 0,
        at,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner { compiled: HashMap::new(), trans: Transitions::default(), states: vec![], start_states: vec![], stack: vec![], flush_count: 0, size: 0 },
    };
    
    let (empty_flags, state_flags) = fsm.start_flags(text, at);
    
    assert_eq!(empty_flags.start, false);
    assert_eq!(empty_flags.end, false);
    assert_eq!(empty_flags.start_line, false);
    assert_eq!(empty_flags.end_line, false);
    assert_eq!(empty_flags.word_boundary, true);
    assert_eq!(empty_flags.not_word_boundary, false);
    assert!(state_flags.is_word());
}

#[test]
fn test_start_flags_at_one_word_boundary() {
    let text = b" Hello";
    let at = 1; // at > 0, at < text.len()
    
    let fsm = Fsm {
        prog: &Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: true, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024},
        start: 0,
        at,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner { compiled: HashMap::new(), trans: Transitions::default(), states: vec![], start_states: vec![], stack: vec![], flush_count: 0, size: 0 },
    };
    
    let (empty_flags, state_flags) = fsm.start_flags(text, at);
    
    assert_eq!(empty_flags.start, false);
    assert_eq!(empty_flags.end, false);
    assert_eq!(empty_flags.start_line, false);
    assert_eq!(empty_flags.end_line, false);
    assert_eq!(empty_flags.word_boundary, true);
    assert_eq!(empty_flags.not_word_boundary, false);
    assert!(state_flags.is_word());
} 

#[test]
fn test_start_flags_at_with_punctuation() {
    let text = b"Hello, world!";
    let at = 6; // at > 0, at < text.len()
    
    let fsm = Fsm {
        prog: &Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: true, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024},
        start: 0,
        at,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner { compiled: HashMap::new(), trans: Transitions::default(), states: vec![], start_states: vec![], stack: vec![], flush_count: 0, size: 0 },
    };
    
    let (empty_flags, state_flags) = fsm.start_flags(text, at);
    
    assert_eq!(empty_flags.start, false);
    assert_eq!(empty_flags.end, false);
    assert_eq!(empty_flags.start_line, false);
    assert_eq!(empty_flags.end_line, false);
    assert_eq!(empty_flags.word_boundary, false);
    assert_eq!(empty_flags.not_word_boundary, true);
    assert!(!state_flags.is_word());
}

