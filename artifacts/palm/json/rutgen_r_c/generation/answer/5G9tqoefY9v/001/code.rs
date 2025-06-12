// Answer 0

#[test]
fn test_skip_to_escape_empty_slice() {
    let mut reader = SliceRead::new(&[]);
    reader.skip_to_escape(false);
    assert_eq!(reader.index, 0);
}

#[test]
fn test_skip_to_escape_at_end_of_slice() {
    let data = b"Hello";
    let mut reader = SliceRead::new(data);
    reader.index = data.len(); // Set index to the end

    reader.skip_to_escape(false);
    assert_eq!(reader.index, data.len());
}

#[test]
fn test_skip_to_escape_consecutive_escapes() {
    let data = b"\\u041b\\u0435";
    let mut reader = SliceRead::new(data);
    reader.index = 0; // Starting at the beginning
    reader.skip_to_escape(false);
    assert_eq!(reader.index, 1); // Should skip over the first escape
}

#[test]
fn test_skip_to_escape_find_first_quote() {
    let data = b"Hello \" World";
    let mut reader = SliceRead::new(data);
    reader.index = 0; // Starting at the beginning
    reader.skip_to_escape(true);
    assert_eq!(reader.index, 6); // Should skip to the quote
}

#[test]
fn test_skip_to_escape_find_first_backslash() {
    let data = b"Hello \\ World";
    let mut reader = SliceRead::new(data);
    reader.index = 0; // Starting at the beginning
    reader.skip_to_escape(true);
    assert_eq!(reader.index, 6); // Should skip to the backslash
}

#[test]
fn test_skip_to_escape_find_first_control_character() {
    let data = b"Hello \x01 World"; // Control character \x01
    let mut reader = SliceRead::new(data);
    reader.index = 0; // Starting at the beginning
    reader.skip_to_escape(true);
    assert_eq!(reader.index, 6); // Should skip to the control character
}

