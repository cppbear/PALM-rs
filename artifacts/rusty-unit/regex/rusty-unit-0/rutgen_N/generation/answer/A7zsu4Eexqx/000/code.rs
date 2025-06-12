// Answer 0

#[test]
fn test_is_valid_cap_letter() {
    assert_eq!(is_valid_cap_letter(&b'a'), true);
    assert_eq!(is_valid_cap_letter(&b'Z'), true);
    assert_eq!(is_valid_cap_letter(&b'9'), true);
    assert_eq!(is_valid_cap_letter(&b'_'), true);
    assert_eq!(is_valid_cap_letter(&b'!'), false);
    assert_eq!(is_valid_cap_letter(&b' '), false);
}

#[test]
fn test_is_valid_cap_letter_boundary() {
    assert_eq!(is_valid_cap_letter(&b'0'), true);
    assert_eq!(is_valid_cap_letter(&b'9'), true);
    assert_eq!(is_valid_cap_letter(&b'a'), true);
    assert_eq!(is_valid_cap_letter(&b'z'), true);
    assert_eq!(is_valid_cap_letter(&b'A'), true);
    assert_eq!(is_valid_cap_letter(&b'Z'), true);
}

