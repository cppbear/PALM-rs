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
fn test_unicode_literal() {
    let unicode_literal = Literal::Unicode('a');
    assert!(unicode_literal.is_unicode());
}

#[test]
fn test_byte_literal_ascii() {
    let byte_literal_ascii = Literal::Byte(0x7F);
    assert!(byte_literal_ascii.is_unicode());
}

#[test]
fn test_byte_literal_non_ascii() {
    let byte_literal_non_ascii = Literal::Byte(0x80);
    assert!(!byte_literal_non_ascii.is_unicode());
}

