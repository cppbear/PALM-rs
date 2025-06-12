// Answer 0

#[test]
fn test_start_flags_reverse_middle_of_word() {
    struct CacheInner { /* Dummy fields if necessary */ }
    struct Program { /* Dummy fields if necessary */ }
    
    let text = b"hello world";
    let fsm = Fsm {
        prog: &Program {},
        start: 0,
        at: 5,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {},
    };

    let (empty_flags, state_flags) = fsm.start_flags_reverse(text.as_ref(), 5);

    assert!(!empty_flags.start);
    assert!(!empty_flags.end);
    assert!(!empty_flags.start_line);
    assert!(!empty_flags.end_line);
    assert!(empty_flags.word_boundary);
    assert!(!empty_flags.not_word_boundary);
    assert!(state_flags.is_word());
}

#[test]
fn test_start_flags_reverse_at_start_of_word() {
    struct CacheInner { /* Dummy fields if necessary */ }
    struct Program { /* Dummy fields if necessary */ }
    
    let text = b"hello world";
    let fsm = Fsm {
        prog: &Program {},
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {},
    };

    let (empty_flags, state_flags) = fsm.start_flags_reverse(text.as_ref(), 0);
    
    assert!(!empty_flags.start);
    assert!(empty_flags.end);
    assert!(empty_flags.start_line);
    assert!(empty_flags.end_line);
    assert!(!empty_flags.word_boundary);
    assert!(empty_flags.not_word_boundary);
    assert!(!state_flags.is_word());
}

#[test]
fn test_start_flags_reverse_one_character() {
    struct CacheInner { /* Dummy fields if necessary */ }
    struct Program { /* Dummy fields if necessary */ }
    
    let text = b"a";
    let fsm = Fsm {
        prog: &Program {},
        start: 0,
        at: 1,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {},
    };

    let (empty_flags, state_flags) = fsm.start_flags_reverse(text.as_ref(), 1);
    
    assert!(empty_flags.start);
    assert!(!empty_flags.end);
    assert!(empty_flags.start_line);
    assert!(!empty_flags.end_line);
    assert!(empty_flags.word_boundary);
    assert!(!empty_flags.not_word_boundary);
    assert!(state_flags.is_word());
}

#[test]
fn test_start_flags_reverse_empty_string() {
    struct CacheInner { /* Dummy fields if necessary */ }
    struct Program { /* Dummy fields if necessary */ }
    
    let text: &[u8] = b"";
    let fsm = Fsm {
        prog: &Program {},
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {},
    };

    let (empty_flags, state_flags) = fsm.start_flags_reverse(text, 0);
    
    assert!(empty_flags.start);
    assert!(empty_flags.end);
    assert!(empty_flags.start_line);
    assert!(empty_flags.end_line);
    assert!(!empty_flags.word_boundary);
    assert!(empty_flags.not_word_boundary);
    assert!(!state_flags.is_word());
}

