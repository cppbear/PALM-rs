// Answer 0

#[test]
fn test_literal_is_unicode_with_byte_greater_than_127() {
    let literal = Literal::Byte(200); // b > 0x7F
    assert_eq!(literal.is_unicode(), false);
}

#[test]
fn test_literal_is_unicode_with_byte_127() {
    let literal = Literal::Byte(127); // b == 0x7F
    assert_eq!(literal.is_unicode(), true);
}

#[test]
fn test_literal_is_unicode_with_unicode_char() {
    let literal = Literal::Unicode('a'); // Unicode scalar value
    assert_eq!(literal.is_unicode(), true);
}

#[test]
fn test_literal_is_unicode_with_byte_0() {
    let literal = Literal::Byte(0); // b <= 0x7F
    assert_eq!(literal.is_unicode(), true);
}

