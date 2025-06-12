// Answer 0

#[test]
fn test_start_flags_reverse_at_equal_text_length() {
    let text: &[u8] = b"Hello, world!";
    let at = text.len();
    
    let fsm = Fsm {
        prog: &Program::default(),
        start: STATE_START,
        at,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };
    
    let (empty_flags, state_flags) = fsm.start_flags_reverse(text, at);
    
    assert_eq!(empty_flags.start, true);
    assert_eq!(empty_flags.end, false);
    assert_eq!(empty_flags.start_line, true);
    assert_eq!(empty_flags.end_line, false);
}

#[test]
fn test_start_flags_reverse_at_less_than_text_length() {
    let text: &[u8] = b"Hello, world!";
    let at = text.len() - 1;

    let fsm = Fsm {
        prog: &Program::default(),
        start: STATE_START,
        at,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };

    let (empty_flags, state_flags) = fsm.start_flags_reverse(text, at);
    
    assert_eq!(empty_flags.start, false);
    assert_eq!(empty_flags.end, false);
    assert_eq!(empty_flags.start_line, false);
    assert_eq!(empty_flags.end_line, false);

    assert!(state_flags.is_word());
}

#[test]
fn test_start_flags_reverse_at_greater_than_zero() {
    let text: &[u8] = b"Hello, world!";
    let at = 5; // The character 'o' in "Hello"

    let fsm = Fsm {
        prog: &Program::default(),
        start: STATE_START,
        at,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };

    let (empty_flags, state_flags) = fsm.start_flags_reverse(text, at);
    
    assert_eq!(empty_flags.start, false);
    assert_eq!(empty_flags.end, false);
    assert_eq!(empty_flags.start_line, false);
    assert_eq!(empty_flags.end_line, false);
    
    assert!(state_flags.is_word());
}

#[test]
fn test_start_flags_reverse_is_word_last_true() {
    let text: &[u8] = b"Hello";
    let at = 4; // The character 'o'

    let fsm = Fsm {
        prog: &Program::default(),
        start: STATE_START,
        at,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };

    let (empty_flags, state_flags) = fsm.start_flags_reverse(text, at);
    
    assert_eq!(empty_flags.start, false);
    assert_eq!(empty_flags.end, false);
    assert_eq!(empty_flags.start_line, false);
    assert_eq!(empty_flags.end_line, false);
    
    assert!(state_flags.is_word());
}

#[test]
fn test_start_flags_reverse_is_word_true() {
    let text: &[u8] = b"Hello!";
    let at = 4; // The exclamation point '!', which is not a word character

    let fsm = Fsm {
        prog: &Program::default(),
        start: STATE_START,
        at,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };

    let (empty_flags, state_flags) = fsm.start_flags_reverse(text, at);
    
    assert_eq!(empty_flags.start, false);
    assert_eq!(empty_flags.end, false);
    assert_eq!(empty_flags.start_line, false);
    assert_eq!(empty_flags.end_line, false);

    assert!(!state_flags.is_word());
}

