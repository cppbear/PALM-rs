// Answer 0

#[test]
fn test_state_flags_debug_struct() {
    let flags = StateFlags(0b00000000);
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);

    flags.fmt(formatter).unwrap();
    assert!(buffer.contains("is_match: false"));
    assert!(buffer.contains("is_word: false"));
    assert!(buffer.contains("has_empty: false"));
}

#[test]
fn test_state_flags_is_match_true() {
    let mut flags = StateFlags(0b00000001);
    assert!(flags.is_match());
}

#[test]
fn test_state_flags_is_match_false() {
    let flags = StateFlags(0b00000000);
    assert!(!flags.is_match());
}

#[test]
fn test_state_flags_is_word_true() {
    let mut flags = StateFlags(0b00000010);
    assert!(flags.is_word());
}

#[test]
fn test_state_flags_is_word_false() {
    let flags = StateFlags(0b00000000);
    assert!(!flags.is_word());
}

#[test]
fn test_state_flags_has_empty_true() {
    let mut flags = StateFlags(0b00000100);
    assert!(flags.has_empty());
}

#[test]
fn test_state_flags_has_empty_false() {
    let flags = StateFlags(0b00000000);
    assert!(!flags.has_empty());
}

