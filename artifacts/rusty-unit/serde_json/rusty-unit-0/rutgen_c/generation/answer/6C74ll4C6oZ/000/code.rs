// Answer 0

#[test]
fn test_parse_index_valid_number() {
    let result = parse_index("42");
    assert_eq!(result, Some(42));
}

#[test]
fn test_parse_index_zero() {
    let result = parse_index("0");
    assert_eq!(result, Some(0));
}

#[test]
fn test_parse_index_leading_zero() {
    let result = parse_index("01");
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_plus_sign() {
    let result = parse_index("+1");
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_non_numeric() {
    let result = parse_index("abc");
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_empty_string() {
    let result = parse_index("");
    assert_eq!(result, None);
}

