// Answer 0

#[test]
fn test_parse_index_valid() {
    assert_eq!(parse_index("123"), Some(123));
    assert_eq!(parse_index("0"), Some(0));
    assert_eq!(parse_index("456"), Some(456));
}

#[test]
fn test_parse_index_invalid_start_plus() {
    assert_eq!(parse_index("+123"), None);
}

#[test]
fn test_parse_index_invalid_start_zero() {
    assert_eq!(parse_index("0123"), None);
}

#[test]
fn test_parse_index_empty_string() {
    assert_eq!(parse_index(""), None);
}

#[test]
fn test_parse_index_non_numeric() {
    assert_eq!(parse_index("abc"), None);
}

