// Answer 0

#[test]
fn test_skip_to_escape_empty_slice() {
    let mut read = SliceRead::new(&[]);
    read.skip_to_escape(false);
    assert_eq!(read.index, 0);
}

#[test]
fn test_skip_to_escape_no_escape_characters() {
    let mut read = SliceRead::new(b"hello world");
    read.skip_to_escape(true);
    assert_eq!(read.index, 11);
}

#[test]
fn test_skip_to_escape_with_escape_character() {
    let mut read = SliceRead::new(b"hello \\ world");
    read.skip_to_escape(true);
    assert_eq!(read.index, 5); // Should stop before the backslash
}

#[test]
fn test_skip_to_escape_with_control_characters() {
    let mut read = SliceRead::new(b"hello \x01 world");
    read.skip_to_escape(true);
    assert_eq!(read.index, 5); // Stops at control character
}

#[test]
fn test_skip_to_escape_with_quote_character() {
    let mut read = SliceRead::new(b"hello \" world");
    read.skip_to_escape(true);
    assert_eq!(read.index, 5); // Should stop before the quote
}

#[test]
fn test_skip_to_escape_successful_with_no_for_ex() {
    let mut read = SliceRead::new(b"hello \" world");
    read.skip_to_escape(false);
    assert_eq!(read.index, 6); // Should find the quote
}

#[test]
fn test_skip_to_escape_with_consecutive_backslashes() {
    let mut read = SliceRead::new(b"\\\\");
    read.skip_to_escape(false);
    assert_eq!(read.index, 1); // Should skip past the first backslash
}

