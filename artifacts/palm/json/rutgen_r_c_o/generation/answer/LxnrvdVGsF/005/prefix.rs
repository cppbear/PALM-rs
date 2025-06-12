// Answer 0

#[test]
fn test_ignore_str_with_valid_input() {
    let mut scratch = Vec::new();
    let input = &[b'a', b'b', b'c', b'"', b'd', b'e'];
    let mut reader = SliceRead::new(input);
    reader.index = 3; // position right before the quote
    let _ = reader.ignore_str();
}

#[test]
fn test_ignore_str_with_escape_character() {
    let mut scratch = Vec::new();
    let input = &[b'a', b'b', b'\\', b'"', b'd', b'e'];
    let mut reader = SliceRead::new(input);
    reader.index = 3; // position right before the quote
    let _ = reader.ignore_str();
}

#[test]
fn test_ignore_str_single_character_before_quote() {
    let mut scratch = Vec::new();
    let input = &[b'"'];
    let mut reader = SliceRead::new(input);
    reader.index = 0; // position right at the start which is also the quote
    let _ = reader.ignore_str();
}

