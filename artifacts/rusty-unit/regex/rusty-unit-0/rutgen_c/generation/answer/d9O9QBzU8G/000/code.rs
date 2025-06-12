// Answer 0

#[test]
fn test_is_hex_with_lowercase_hex_digits() {
    assert_eq!(is_hex('a'), true);
    assert_eq!(is_hex('f'), true);
}

#[test]
fn test_is_hex_with_uppercase_hex_digits() {
    assert_eq!(is_hex('A'), true);
    assert_eq!(is_hex('F'), true);
}

#[test]
fn test_is_hex_with_numeric_digits() {
    assert_eq!(is_hex('0'), true);
    assert_eq!(is_hex('9'), true);
}

#[test]
fn test_is_hex_with_non_hex_characters() {
    assert_eq!(is_hex('g'), false);
    assert_eq!(is_hex('Z'), false);
    assert_eq!(is_hex('-'), false);
    assert_eq!(is_hex('!'), false);
}

#[test]
fn test_is_hex_with_boundary_characters() {
    assert_eq!(is_hex(' '), false);
    assert_eq!(is_hex('\n'), false);
    assert_eq!(is_hex('\t'), false);
}

