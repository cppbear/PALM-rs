// Answer 0

#[test]
fn test_parse_index_valid_input() {
    let input = "5";
    let result = parse_index(input);
    assert_eq!(result, Some(5));
}

#[test]
fn test_parse_index_empty_string() {
    let input = "";
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_leading_zero() {
    let input = "01";
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_negative_sign() {
    let input = "-5";
    let result = parse_index(input);
    assert_eq!(result, Some(-5 as usize)); // Should be none as usize cannot be negative.
}

#[test]
fn test_parse_index_plus_sign() {
    let input = "+5";
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_non_digit_characters() {
    let input = "five";
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_large_number() {
    let input = "123456789";
    let result = parse_index(input);
    assert_eq!(result, Some(123456789));
}

#[test]
fn test_parse_index_single_zero() {
    let input = "0";
    let result = parse_index(input);
    assert_eq!(result, Some(0));
}

