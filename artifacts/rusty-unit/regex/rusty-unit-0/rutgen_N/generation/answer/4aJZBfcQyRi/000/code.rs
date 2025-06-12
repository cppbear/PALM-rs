// Answer 0

#[test]
fn test_start_flags_reverse_empty_text() {
    struct TestDFA;

    let dfa = TestDFA;
    let text: &[u8] = b"";
    let at = 0;

    let (empty_flags, state_flags) = dfa.start_flags_reverse(text, at);
    
    assert!(empty_flags.start);
    assert!(empty_flags.end);
    assert!(empty_flags.start_line);
    assert!(empty_flags.end_line);
    assert!(!empty_flags.word_boundary);
    assert!(empty_flags.not_word_boundary);
    assert_eq!(state_flags, StateFlags::default());
}

#[test]
fn test_start_flags_reverse_at_end() {
    struct TestDFA;

    let dfa = TestDFA;
    let text: &[u8] = b"hello";
    let at = 5;

    let (empty_flags, state_flags) = dfa.start_flags_reverse(text, at);
    
    assert!(empty_flags.start);
    assert!(!empty_flags.end);
    assert!(empty_flags.start_line);
    assert!(!empty_flags.end_line);
    assert!(!empty_flags.word_boundary);
    assert!(empty_flags.not_word_boundary);
    assert_eq!(state_flags, StateFlags::default());
}

#[test]
fn test_start_flags_reverse_at_word_boundary() {
    struct TestDFA;

    let dfa = TestDFA;
    let text: &[u8] = b"hello world";
    let at = 5;

    let (empty_flags, state_flags) = dfa.start_flags_reverse(text, at);
    
    assert!(!empty_flags.start);
    assert!(!empty_flags.end);
    assert!(!empty_flags.start_line);
    assert!(!empty_flags.end_line);
    assert!(empty_flags.word_boundary);
    assert!(!empty_flags.not_word_boundary);
    assert!(state_flags.is_word());
}

#[test]
fn test_start_flags_reverse_at_character() {
    struct TestDFA;

    let dfa = TestDFA;
    let text: &[u8] = b"hello, world!";
    let at = 6; // Comma is ASCII punctuation

    let (empty_flags, state_flags) = dfa.start_flags_reverse(text, at);
    
    assert!(!empty_flags.start);
    assert!(!empty_flags.end);
    assert!(!empty_flags.start_line);
    assert!(!empty_flags.end_line);
    assert!(!empty_flags.word_boundary);
    assert!(empty_flags.not_word_boundary);
    assert!(!state_flags.is_word());
}

#[test]
fn test_start_flags_reverse_boundary_conditions() {
    struct TestDFA;

    let dfa = TestDFA;
    let text: &[u8] = b"word";
    
    // Test at the start of a word
    let (empty_flags_start, state_flags_start) = dfa.start_flags_reverse(text, 0);
    assert!(empty_flags_start.word_boundary);
    
    // Test at the end of a word
    let (empty_flags_end, state_flags_end) = dfa.start_flags_reverse(text, 4);
    assert!(empty_flags_end.word_boundary);
}

