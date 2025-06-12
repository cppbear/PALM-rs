// Answer 0

#[test]
fn test_octal_enabled() {
    let mut builder = ParserBuilder::new();
    builder.octal(true);
    assert!(builder.octal); // Assuming there's a way to check the internal state.
}

#[test]
fn test_octal_disabled() {
    let mut builder = ParserBuilder::new();
    builder.octal(false);
    assert!(!builder.octal); // Assuming there's a way to check the internal state.
}

#[test]
fn test_octal_state_persistence() {
    let mut builder = ParserBuilder::new();
    builder.octal(true);
    let current_state = builder.octal; // Store the current state.
    builder.octal(false); // Change state.
    assert_ne!(builder.octal, current_state); // Ensure state change.
}

