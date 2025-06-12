// Answer 0

#[test]
fn test_is_hex_with_zero() {
    assert_eq!(is_hex('0'), true);
}

#[test]
fn test_is_hex_with_nine() {
    assert_eq!(is_hex('9'), true);
}

#[test]
fn test_is_hex_with_a() {
    assert_eq!(is_hex('a'), true);
}

#[test]
fn test_is_hex_with_f() {
    assert_eq!(is_hex('f'), true);
}

#[test]
fn test_is_hex_with_A() {
    assert_eq!(is_hex('A'), false);
}

#[test]
fn test_is_hex_with_G() {
    assert_eq!(is_hex('G'), false);
}

#[test]
fn test_is_hex_with_space() {
    assert_eq!(is_hex(' '), false);
}

#[test]
fn test_is_hex_with_invalid_char() {
    assert_eq!(is_hex('&'), false);
}

