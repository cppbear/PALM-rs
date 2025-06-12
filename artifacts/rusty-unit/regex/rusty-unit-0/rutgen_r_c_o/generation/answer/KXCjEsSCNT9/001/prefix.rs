// Answer 0

#[test]
fn test_literal_is_unicode_byte_0() {
    let literal = Literal::Byte(0);
    literal.is_unicode();
}

#[test]
fn test_literal_is_unicode_byte_1() {
    let literal = Literal::Byte(1);
    literal.is_unicode();
}

#[test]
fn test_literal_is_unicode_byte_2() {
    let literal = Literal::Byte(2);
    literal.is_unicode();
}

#[test]
fn test_literal_is_unicode_byte_63() {
    let literal = Literal::Byte(63);
    literal.is_unicode();
}

#[test]
fn test_literal_is_unicode_byte_127() {
    let literal = Literal::Byte(127);
    literal.is_unicode();
}

