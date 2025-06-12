// Answer 0

#[test]
fn test_set_word_initial_state() {
    let mut flags = StateFlags::default();
    flags.set_word();
    assert_eq!(flags.0, 0b000000_1_0);
}

#[test]
fn test_set_word_multiple_calls() {
    let mut flags = StateFlags::default();
    flags.set_word();
    flags.set_word(); // Call it again
    assert_eq!(flags.0, 0b000000_1_0); // Should not change state on subsequent calls
}

#[test]
fn test_set_word_bitwise() {
    let mut flags = StateFlags(0);
    flags.set_word();
    assert!(flags.0 & 0b000000_1_0 != 0, "The word bit should be set");
}

#[test]
fn test_set_word_combined_flags() {
    let mut flags = StateFlags(0b000000_1_1); // Initial state with some other flag set
    flags.set_word();
    assert_eq!(flags.0, 0b000000_1_1); // Ensure the word flag is set while keeping the other
}

