// Answer 0

#[test]
fn test_parse_index_positive_case() {
    let input = "0"; // Fulfills constraint: s.len() != 1 is false
    let result = parse_index(input);
    assert_eq!(result, Some(0));
}

#[test]
fn test_parse_index_negative_case_starts_with_plus() {
    let input = "+1"; // Fulfills constraint: s.starts_with('+') is true
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_negative_case_starts_with_zero() {
    let input = "01"; // Fulfills constraint: s.starts_with('0') is true && s.len() != 1 is true
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_invalid_string() {
    let input = "abc"; // Non-numeric input
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_empty_string() {
    let input = ""; // Empty string input
    let result = parse_index(input);
    assert_eq!(result, None);
}

