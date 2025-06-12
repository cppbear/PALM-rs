// Answer 0

#[derive(Debug)]
enum Literal {
    Unicode(char),
    Byte(u8),
}

impl Literal {
    pub fn is_unicode(&self) -> bool {
        match *self {
            Literal::Unicode(_) => true,
            Literal::Byte(b) if b <= 0x7F => true,
            Literal::Byte(_) => false,
        }
    }
}

#[test]
fn test_is_unicode_byte_upper_bound() {
    let literal = Literal::Byte(0x7F);
    assert_eq!(literal.is_unicode(), true);
}

#[test]
fn test_is_unicode_byte_within_bounds() {
    let literal = Literal::Byte(0x00);
    assert_eq!(literal.is_unicode(), true);
}

#[test]
fn test_is_unicode_unicode_literal() {
    let literal = Literal::Unicode('a');
    assert_eq!(literal.is_unicode(), true);
}

