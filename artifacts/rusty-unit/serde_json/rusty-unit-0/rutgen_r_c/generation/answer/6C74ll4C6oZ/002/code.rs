// Answer 0

#[test]
fn test_parse_index_starts_with_plus() {
    let input = "+123";
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_starts_with_zero_length_greater_than_one() {
    let input = "0123";
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_starts_with_zero_single_character() {
    let input = "0";
    let result = parse_index(input);
    assert_eq!(result, Some(0));
}

#[test]
fn test_parse_index_empty_string() {
    let input = "";
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_valid_number() {
    let input = "123";
    let result = parse_index(input);
    assert_eq!(result, Some(123));
}

