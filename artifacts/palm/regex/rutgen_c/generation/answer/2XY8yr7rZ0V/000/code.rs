// Answer 0

#[test]
fn test_set_empty() {
    let mut flags = StateFlags::default();
    assert!(!flags.has_empty());
    flags.set_empty();
    assert!(flags.has_empty());
}

#[test]
fn test_set_empty_multiple() {
    let mut flags = StateFlags::default();
    flags.set_empty();
    flags.set_empty(); // Calling again to test idempotency
    assert!(flags.has_empty());
}

