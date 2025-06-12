// Answer 0

#[test]
fn test_set_word() {
    // Initialize the StateFlags with its default constructor
    let mut flags = StateFlags::default();

    // Confirm that the initial state does not set the word flag
    assert!(!flags.is_word());

    // Call the set_word method
    flags.set_word();

    // Verify that the word flag is now set
    assert!(flags.is_word());
}

#[test]
fn test_set_word_multiple_calls() {
    // Initialize the StateFlags with its default constructor
    let mut flags = StateFlags::default();

    // Call set_word multiple times
    flags.set_word();
    flags.set_word();

    // Ensure that the state remains consistent and the flag is still set
    assert!(flags.is_word());
}

#[test]
fn test_set_word_with_custom_flag() {
    // Initialize the StateFlags with a custom state where the word flag is not set
    let mut custom_flags = StateFlags(0b000000_0_0); // Word flag is off

    // Set the word flag
    custom_flags.set_word();

    // Check if the word flag is set after calling set_word
    assert!(custom_flags.is_word());
}

#[test]
fn test_set_word_check_other_flags() {
    // Initialize the StateFlags with a state where other bits are set
    let mut flags = StateFlags(0b000000_1_1); // Arbitrary other flags

    // Call set_word, it should only affect the word related flag
    flags.set_word();

    // Verify that the word flag is set, while other flags remain unaffected
    assert!(flags.is_word());
    assert_eq!(flags.0 & 0b000000_1_1, 0b000000_1_1);
}

