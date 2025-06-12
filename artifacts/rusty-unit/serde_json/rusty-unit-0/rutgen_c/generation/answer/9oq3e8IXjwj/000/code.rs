// Answer 0

#[test]
fn test_skip_to_escape_slow_no_escape() {
    let mut reader = SliceRead::new(b"hello world");
    reader.skip_to_escape_slow();
    assert_eq!(reader.index, 11); // End of slice
}

#[test]
fn test_skip_to_escape_slow_with_escape() {
    let mut reader = SliceRead::new(b"hello \\ world");
    reader.skip_to_escape_slow();
    assert_eq!(reader.index, 6); // Position of escape character
}

#[test]
fn test_skip_to_escape_slow_with_control_characters() {
    let mut reader = SliceRead::new(b"hello \0 world"); // '\0' is a control character
    reader.skip_to_escape_slow();
    assert_eq!(reader.index, 5); // Stops at control character
}

#[test]
fn test_skip_to_escape_slow_empty_slice() {
    let mut reader = SliceRead::new(b"");
    reader.skip_to_escape_slow();
    assert_eq!(reader.index, 0); // No bytes to read
}

