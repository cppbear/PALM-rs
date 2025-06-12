// Answer 0

#[test]
fn test_is_unicode_unicode_literal() {
    let literal = Literal::Unicode('a');
    assert!(literal.is_unicode());
}

#[test]
fn test_is_unicode_byte_lower_bound() {
    let literal = Literal::Byte(0x7F);
    assert!(literal.is_unicode());
}

#[test]
fn test_is_unicode_byte_above_lower_bound() {
    let literal = Literal::Byte(0x41); // 'A'
    assert!(literal.is_unicode());
}

#[test]
fn test_is_unicode_byte_upper_bound() {
    let literal = Literal::Byte(0x80);
    assert!(!literal.is_unicode());
}

#[test]
fn test_is_unicode_byte_over_upper_bound() {
    let literal = Literal::Byte(0xFF);
    assert!(!literal.is_unicode());
}

