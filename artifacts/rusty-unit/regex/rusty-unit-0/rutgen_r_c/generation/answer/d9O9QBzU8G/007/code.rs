// Answer 0

#[test]
fn test_is_hex_lowercase_a() {
    let c = 'a';
    assert_eq!(is_hex(c), true);
}

#[test]
fn test_is_hex_lowercase_f() {
    let c = 'f';
    assert_eq!(is_hex(c), true);
}

#[test]
fn test_is_hex_invalid_lowercase_g() {
    let c = 'g';
    assert_eq!(is_hex(c), false);
}

#[test]
fn test_is_hex_uppercase_a() {
    let c = 'A';
    assert_eq!(is_hex(c), true);
}

#[test]
fn test_is_hex_uppercase_F() {
    let c = 'F';
    assert_eq!(is_hex(c), true);
}

#[test]
fn test_is_hex_invalid_uppercase_G() {
    let c = 'G';
    assert_eq!(is_hex(c), false);
}

#[test]
fn test_is_hex_digit_0() {
    let c = '0';
    assert_eq!(is_hex(c), true);
}

#[test]
fn test_is_hex_digit_9() {
    let c = '9';
    assert_eq!(is_hex(c), true);
}

#[test]
fn test_is_hex_invalid_non_hex_character() {
    let c = 'x';
    assert_eq!(is_hex(c), false);
}

