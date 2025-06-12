// Answer 0

#[test]
fn test_char_input_len_non_empty() {
    let input = CharInput(b"hello");
    assert_eq!(input.len(), 5);
}

#[test]
fn test_char_input_len_empty() {
    let input = CharInput(b"");
    assert_eq!(input.len(), 0);
}

#[test]
fn test_char_input_len_with_unicode() {
    let input = CharInput(b"\xe2\x9c\x94"); // Unicode check mark
    assert_eq!(input.len(), 3);
}

