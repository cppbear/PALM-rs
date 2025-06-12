// Answer 0

#[test]
fn test_parse_index_valid_zero() {
    let input = "0";
    let result = parse_index(input);
    assert_eq!(result, Some(0));
}

#[test]
fn test_parse_index_invalid_positive_check() {
    let input = "+1";
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_invalid_zero_length_check() {
    let input = "00";
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_negative() {
    let input = "-1";
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_string() {
    let input = "abc";
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_empty_string() {
    let input = "";
    let result = parse_index(input);
    assert_eq!(result, None);
}

