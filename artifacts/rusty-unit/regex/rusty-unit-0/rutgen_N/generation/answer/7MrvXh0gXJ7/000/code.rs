// Answer 0

#[test]
fn test_start_flags_beginning_of_text() {
    struct TestDFA;

    let dfa = TestDFA;
    let text = b"hello";
    let at = 0;

    let (empty_flags, state_flags) = dfa.start_flags(text, at);
    
    assert!(empty_flags.start);
    assert!(!empty_flags.end);
    assert!(empty_flags.start_line);
    assert!(empty_flags.end_line);
    assert!(empty_flags.word_boundary);
    assert!(!empty_flags.not_word_boundary);
    assert!(state_flags.is_word());
}

#[test]
fn test_start_flags_end_of_text() {
    struct TestDFA;

    let dfa = TestDFA;
    let text = b"hello";
    let at = text.len();

    let (empty_flags, state_flags) = dfa.start_flags(text, at);
    
    assert!(!empty_flags.start);
    assert!(empty_flags.end);
    assert!(!empty_flags.start_line);
    assert!(empty_flags.end_line);
    assert!(!empty_flags.word_boundary);
    assert!(!empty_flags.not_word_boundary);
    assert!(!state_flags.is_word());
}

#[test]
fn test_start_flags_inside_word() {
    struct TestDFA;

    let dfa = TestDFA;
    let text = b"hello world";
    let at = 5; // Position between "hello" and " "

    let (empty_flags, state_flags) = dfa.start_flags(text, at);
    
    assert!(!empty_flags.start);
    assert!(!empty_flags.end);
    assert!(!empty_flags.start_line);
    assert!(!empty_flags.end_line);
    assert!(empty_flags.word_boundary);
    assert!(!empty_flags.not_word_boundary);
    assert!(state_flags.is_word());
}

#[test]
fn test_start_flags_word_boundary() {
    struct TestDFA;

    let dfa = TestDFA;
    let text = b"hello world";
    let at = 6; // Position on the space

    let (empty_flags, state_flags) = dfa.start_flags(text, at);
    
    assert!(!empty_flags.start);
    assert!(!empty_flags.end);
    assert!(!empty_flags.start_line);
    assert!(!empty_flags.end_line);
    assert!(!empty_flags.word_boundary);
    assert!(empty_flags.not_word_boundary);
    assert!(!state_flags.is_word());
}

#[test]
fn test_start_flags_line_start() {
    struct TestDFA;

    let dfa = TestDFA;
    let text = b"\nhello";
    let at = 0;

    let (empty_flags, state_flags) = dfa.start_flags(text, at);
    
    assert!(empty_flags.start);
    assert!(!empty_flags.end);
    assert!(empty_flags.start_line);
    assert!(!empty_flags.end_line);
}

#[test]
fn test_start_flags_empty_text() {
    struct TestDFA;

    let dfa = TestDFA;
    let text: &[u8] = b"";
    let at = 0;

    let (empty_flags, state_flags) = dfa.start_flags(text, at);
    
    assert!(empty_flags.start);
    assert!(empty_flags.end);
    assert!(empty_flags.start_line);
    assert!(empty_flags.end_line);
}

