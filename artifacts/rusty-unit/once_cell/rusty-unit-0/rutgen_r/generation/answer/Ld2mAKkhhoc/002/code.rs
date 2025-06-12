// Answer 0

#[test]
fn test_set_when_cell_is_empty() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    assert!(cell.get().is_none()); // Check that the cell is initially empty

    // Attempt to set a value when the cell is empty
    assert_eq!(cell.set(100), Ok(())); // Expect Ok(()) as the cell is now set
    assert_eq!(cell.get(), Some(&100)); // Verify the value has been set correctly
}

#[test]
fn test_set_when_cell_is_full() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    assert_eq!(cell.set(200), Ok(())); // First set should succeed
    assert_eq!(cell.set(300), Err(300)); // Second set should fail and return the value being set

    // Verify that the cell still holds the first value
    assert_eq!(cell.get(), Some(&200));
}

#[test]
fn test_set_multiple_consecutive_attempts() {
    use once_cell::unsync::OnceCell;

    let cell = OnceCell::new();
    assert_eq!(cell.set("first"), Ok(())); // First set should succeed
    assert_eq!(cell.set("second"), Err("second")); // Second set should fail

    // Verify the cell holds the expected value
    assert_eq!(cell.get(), Some(&"first"));
}

