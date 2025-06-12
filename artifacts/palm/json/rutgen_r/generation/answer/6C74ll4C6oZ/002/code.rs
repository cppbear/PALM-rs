// Answer 0

#[test]
fn test_parse_index_starts_with_plus() {
    let input = "+123";
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_starts_with_zero_and_length_greater_than_one() {
    let input = "0123";
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_starts_with_zero_length_equal_to_one() {
    let input = "0";
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_normal_numbers() {
    let input = "123";
    let result = parse_index(input);
    assert_eq!(result, Some(123));
}

