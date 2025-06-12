// Answer 0

#[test]
fn test_parse_index_valid_zero() {
    let input = "0";
    let result = parse_index(input);
    assert_eq!(result, Some(0));
}

#[test]
fn test_parse_index_invalid_positive() {
    let input = "+1";
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_invalid_leading_zero() {
    let input = "01";
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_empty_string() {
    let input = "";
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_invalid_non_numeric() {
    let input = "abc";
    let result = parse_index(input);
    assert_eq!(result, None);
}

