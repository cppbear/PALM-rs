// Answer 0

#[test]
fn test_parse_index_valid_input() {
    assert_eq!(parse_index("3"), Some(3));
    assert_eq!(parse_index("10"), Some(10));
    assert_eq!(parse_index("12345"), Some(12345));
}

#[test]
fn test_parse_index_zero_input() {
    assert_eq!(parse_index("0"), Some(0));
}

#[test]
fn test_parse_index_invalid_input_starting_with_plus() {
    assert_eq!(parse_index("+1"), None);
}

#[test]
fn test_parse_index_invalid_input_starting_with_zero() {
    assert_eq!(parse_index("01"), None);
    assert_eq!(parse_index("000"), None);
}

#[test]
fn test_parse_index_non_numeric_input() {
    assert_eq!(parse_index("abc"), None);
    assert_eq!(parse_index("12abc"), None);
}

#[test]
fn test_parse_index_empty_string() {
    assert_eq!(parse_index(""), None);
}

#[test]
fn test_parse_index_large_numbers() {
    assert_eq!(parse_index("18446744073709551615"), None); // Exceeds usize
}

