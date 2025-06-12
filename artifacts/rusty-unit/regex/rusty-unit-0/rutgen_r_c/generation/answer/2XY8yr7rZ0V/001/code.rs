// Answer 0

#[test]
fn test_set_empty() {
    // Initialize a StateFlags instance with default value
    let mut state_flags = StateFlags::default();

    // Check initial state (0b00000000)
    assert_eq!(state_flags.0, 0b00000000);

    // Call set_empty to set the empty flag
    state_flags.set_empty();

    // Check if the empty flag has been set (0b00000001)
    assert_eq!(state_flags.0, 0b00000_1_00);
}

#[test]
fn test_set_empty_multiple_calls() {
    // Initialize a StateFlags instance with default value
    let mut state_flags = StateFlags::default();

    // Call set_empty multiple times
    state_flags.set_empty();
    state_flags.set_empty();

    // Ensure the state is still correctly set (0b00000001)
    assert_eq!(state_flags.0, 0b00000_1_00);
}

