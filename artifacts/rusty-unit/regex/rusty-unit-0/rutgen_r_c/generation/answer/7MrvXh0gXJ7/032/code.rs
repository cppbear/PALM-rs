// Answer 0

#[test]
fn test_start_flags_at_zero_empty_text() {
    let text: &[u8] = b"";
    let at: usize = 0;

    let fsm = Fsm {
        prog: &Program::default(),
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };

    let (empty_flags, state_flags) = fsm.start_flags(text, at);
    
    assert!(empty_flags.start);
    assert!(empty_flags.end);
    assert!(empty_flags.start_line);
    assert!(empty_flags.end_line);
    assert!(!empty_flags.word_boundary);
    assert!(!empty_flags.not_word_boundary);
    assert!(!state_flags.is_word());
}

#[test]
fn test_start_flags_not_at_zero_empty_text() {
    let text: &[u8] = b"";
    let at: usize = 1;

    let fsm = Fsm {
        prog: &Program::default(),
        start: STATE_START,
        at: 1,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };

    let (empty_flags, state_flags) = fsm.start_flags(text, at);
    
    assert!(!empty_flags.start);
    assert!(empty_flags.end);
    assert!(empty_flags.start_line);
    assert!(empty_flags.end_line);
    assert!(!empty_flags.word_boundary);
    assert!(!empty_flags.not_word_boundary);
    assert!(!state_flags.is_word());
}

#[test]
fn test_start_flags_at_equal_text_len() {
    let text: &[u8] = b"Hello";
    let at: usize = text.len(); // at == text.len()

    let fsm = Fsm {
        prog: &Program::default(),
        start: STATE_START,
        at: text.len(),
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };

    let (empty_flags, state_flags) = fsm.start_flags(text, at);
    
    assert!(!empty_flags.start);
    assert!(empty_flags.end); // text.is_empty() is false
    assert!(!empty_flags.start_line);
    assert!(!empty_flags.end_line);
    assert!(!empty_flags.word_boundary);
    assert!(!empty_flags.not_word_boundary);
    assert!(!state_flags.is_word());
}

#[test]
fn test_start_flags_word_boundary_conditions() {
    let text: &[u8] = b"Hello\nWorld";
    
    let fsm = Fsm {
        prog: &Program::default(),
        start: STATE_START,
        at: 5,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };

    let (empty_flags, state_flags) = fsm.start_flags(text, 5);
    
    assert!(!empty_flags.word_boundary);
    assert!(!empty_flags.not_word_boundary);
    assert!(state_flags.is_word()); // previous char is 'o'
}

#[test]
fn test_start_flags_not_word_boundary() {
    let text: &[u8] = b"Hello!";
    
    let fsm = Fsm {
        prog: &Program::default(),
        start: STATE_START,
        at: 5,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };

    let (empty_flags, state_flags) = fsm.start_flags(text, 5);
    
    assert!(empty_flags.word_boundary);
    assert!(!empty_flags.not_word_boundary);
    assert!(state_flags.is_word());
}

