// Answer 0

#[test]
fn test_skip_to_escape_with_no_escape_chars() {
    let input = b"Hello, World!";
    let mut reader = SliceRead::new(input);
    reader.index = 0; // Setting index to the start
    reader.skip_to_escape(false);
    assert_eq!(reader.index, input.len()); // No escape characters should cause the index to move to the end
}

#[test]
fn test_skip_to_escape_with_escape_character() {
    let input = b"Hello, World! \"Escape\\ here\"";
    let mut reader = SliceRead::new(input);
    reader.index = 0; // Setting index to the start
    reader.skip_to_escape(false);
    assert!(reader.index < input.len()); // The index should not reach the end
    assert_eq!(input[reader.index], b'"'); // The first escape character found should be a quote
}

#[test]
fn test_skip_to_escape_with_control_characters() {
    let input = b"Hello\x01World!"; // Control character 0x01
    let mut reader = SliceRead::new(input);
    reader.index = 0; // Setting index to the start
    reader.skip_to_escape(true);
    assert!(reader.index < input.len()); // We should stop before the control character is reached
    assert_eq!(input[reader.index], b'\x01'); // Should find the control character
}

#[test]
fn test_skip_to_escape_empty_slice() {
    let input: &[u8] = &[];
    let mut reader = SliceRead::new(input);
    reader.index = 0; // Setting index to the start
    reader.skip_to_escape(false);
    assert_eq!(reader.index, 0); // Should remain at 0 as there is no data
}

#[test]
fn test_skip_to_escape_all_escape_chars() {
    let input = b"\"\\\"\\";
    let mut reader = SliceRead::new(input);
    reader.index = 0; // Set the index to start
    reader.skip_to_escape(false);
    assert_eq!(reader.index, input.len()); // Should skip all, landing at the end
}

