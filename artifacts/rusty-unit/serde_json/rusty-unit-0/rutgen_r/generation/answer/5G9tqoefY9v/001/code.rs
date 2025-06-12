// Answer 0

#[test]
fn test_skip_to_escape_empty_slice() {
    let mut slice = &b""[..];
    let mut index = slice.len();
    let mut forbids_control_characters = false;

    skip_to_escape(&mut index, slice, forbids_control_characters);

    assert_eq!(index, slice.len());
}

#[test]
fn test_skip_to_escape_consecutive_escapes() {
    let slice = &b"\\\\Hello"[..];
    let mut index = 0;
    let forbids_control_characters = false;

    skip_to_escape(&mut index, slice, forbids_control_characters);

    assert_eq!(index, 2); // Index should skip past the first two escape characters
}

#[test]
fn test_skip_to_escape_with_control_characters() {
    let slice = &b"Hello\x00World"[..]; // Contains control character
    let mut index = 0;
    let forbids_control_characters = true;

    skip_to_escape(&mut index, slice, forbids_control_characters);

    assert!(index < slice.len()); // Should stop before the control character
}

#[test]
fn test_skip_to_escape_with_quotes_and_backslashes() {
    let slice = &b"Hello\"World\\Test"[..]; // Contains quote and backslash
    let mut index = 0;
    let forbids_control_characters = true;

    skip_to_escape(&mut index, slice, forbids_control_characters);

    assert!(index < slice.len()); // Should stop at the quote or backslash
}

