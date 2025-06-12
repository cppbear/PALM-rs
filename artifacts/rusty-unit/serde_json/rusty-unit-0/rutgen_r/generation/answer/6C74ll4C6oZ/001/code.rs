// Answer 0

#[test]
fn test_parse_index_starts_with_plus() {
    let input = "+123";
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_starts_with_plus_empty() {
    let input = "+";
    let result = parse_index(input);
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_starts_with_plus_zero() {
    let input = "+0";
    let result = parse_index(input);
    assert_eq!(result, None);
}

