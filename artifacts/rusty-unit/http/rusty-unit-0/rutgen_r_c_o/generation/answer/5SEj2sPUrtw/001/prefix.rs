// Answer 0

#[test]
fn test_from_static_empty() {
    let val = HeaderValue::from_static("");
}

#[test]
fn test_from_static_single_space() {
    let val = HeaderValue::from_static(" ");
}

#[test]
fn test_from_static_valid_single_character() {
    let val = HeaderValue::from_static("A");
}

#[test]
fn test_from_static_valid_string() {
    let val = HeaderValue::from_static("hello");
}

#[test]
fn test_from_static_valid_repeated_characters() {
    let val = HeaderValue::from_static("A".repeat(20).as_str());
}

#[should_panic]
fn test_from_static_invalid_newline() {
    let _ = HeaderValue::from_static("hello\n");
}

#[should_panic]
fn test_from_static_invalid_backspace() {
    let _ = HeaderValue::from_static("hello\x08");
}

#[should_panic]
fn test_from_static_invalid_delete() {
    let _ = HeaderValue::from_static("hello\x7f");
}

#[test]
fn test_from_static_end_boundary() {
    let val = HeaderValue::from_static("~");
}

