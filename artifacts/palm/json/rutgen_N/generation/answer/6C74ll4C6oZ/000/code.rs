// Answer 0

#[test]
fn test_parse_index_valid() {
    assert_eq!(parse_index("0"), Some(0));
    assert_eq!(parse_index("1"), Some(1));
    assert_eq!(parse_index("10"), Some(10));
}

#[test]
fn test_parse_index_invalid() {
    assert_eq!(parse_index("+1"), None);
    assert_eq!(parse_index("01"), None);
    assert_eq!(parse_index("abc"), None);
    assert_eq!(parse_index("-1"), None);
}

#[test]
fn test_parse_index_edge_cases() {
    assert_eq!(parse_index(""), None);
    assert_eq!(parse_index("+"), None);
}

