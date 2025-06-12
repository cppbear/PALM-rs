// Answer 0

#[test]
fn test_parse_index_starts_with_plus() {
    let result = parse_index("+1");
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_starts_with_plus_empty() {
    let result = parse_index("+");
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_starts_with_plus_space() {
    let result = parse_index("+ ");
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_starts_with_plus_non_numeric() {
    let result = parse_index("+abc");
    assert_eq!(result, None);
}

#[test]
fn test_parse_index_starts_with_plus_negative() {
    let result = parse_index("+-1");
    assert_eq!(result, None);
}

