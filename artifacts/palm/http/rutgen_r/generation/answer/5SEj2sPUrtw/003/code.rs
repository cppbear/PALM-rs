// Answer 0

#[test]
fn test_from_static_valid_input() {
    let val = http::header::HeaderValue::from_static("valid_header_value");
    assert_eq!(&val.inner[..], b"valid_header_value");
    assert!(!val.is_sensitive);
}

#[test]
#[should_panic(expected = "any use of this value will cause an error")]
fn test_from_static_invalid_character() {
    let _ = http::header::HeaderValue::from_static("invalid\0character");
}

#[test]
#[should_panic(expected = "any use of this value will cause an error")]
fn test_from_static_invalid_visible_ascii() {
    let _ = http::header::HeaderValue::from_static("жinvalid_character");
}

#[test]
fn test_from_static_empty_string() {
    let val = http::header::HeaderValue::from_static("");
    assert_eq!(&val.inner[..], b"");
    assert!(!val.is_sensitive);
}

#[test]
fn test_from_static_only_space() {
    let val = http::header::HeaderValue::from_static(" ");
    assert_eq!(&val.inner[..], b" ");
    assert!(!val.is_sensitive);
}

#[test]
fn test_from_static_boundary_non_ascii() {
    let _ = http::header::HeaderValue::from_static("valid_header_value_with_invalid_character_ж");
}

