// Answer 0

#[test]
fn test_is_unicode_with_unicode_literal() {
    let unicode_literal = Literal::Unicode('A');
    assert!(unicode_literal.is_unicode());
}

#[test]
fn test_is_unicode_with_byte_literal_ascii() {
    let byte_literal = Literal::Byte(65); // 'A' in ASCII
    assert!(byte_literal.is_unicode());
}

#[test]
fn test_is_unicode_with_byte_literal_non_ascii() {
    let byte_literal = Literal::Byte(200); // non-ASCII byte
    assert!(!byte_literal.is_unicode());
}

#[test]
fn test_is_unicode_with_byte_literal_boundary() {
    let byte_literal_boundary = Literal::Byte(127); // maximum ASCII byte
    assert!(byte_literal_boundary.is_unicode());
}

#[test]
fn test_is_unicode_with_byte_literal_negative() {
    let byte_literal_negative = Literal::Byte(255); // outside ASCII range
    assert!(!byte_literal_negative.is_unicode());
}

