// Answer 0

#[test]
fn test_is_unicode_unicode_literal() {
    let literal = Literal::Unicode('a');
    assert!(literal.is_unicode());
}

#[test]
fn test_is_unicode_byte_literal_ascii() {
    let literal = Literal::Byte(0x7F);
    assert!(literal.is_unicode());
}

#[test]
fn test_is_unicode_byte_literal_non_ascii() {
    let literal = Literal::Byte(0x80);
    assert!(!literal.is_unicode());
}

#[test]
fn test_is_unicode_byte_literal_zero() {
    let literal = Literal::Byte(0);
    assert!(literal.is_unicode());
}

