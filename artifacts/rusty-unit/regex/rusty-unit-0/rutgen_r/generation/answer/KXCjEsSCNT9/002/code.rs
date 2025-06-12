// Answer 0

#[test]
fn test_is_unicode_literal_byte_above_7f() {
    struct Literal {
        kind: LiteralKind,
    }

    enum LiteralKind {
        Unicode(char),
        Byte(u8),
    }

    impl Literal {
        pub fn is_unicode(&self) -> bool {
            match self.kind {
                LiteralKind::Unicode(_) => true,
                LiteralKind::Byte(b) if b <= 0x7F => true,
                LiteralKind::Byte(_) => false,
            }
        }
    }

    let literal_above_7f = Literal { kind: LiteralKind::Byte(0x80) };
    assert_eq!(literal_above_7f.is_unicode(), false);
}

#[test]
fn test_is_unicode_literal_byte_exactly_7f() {
    struct Literal {
        kind: LiteralKind,
    }

    enum LiteralKind {
        Unicode(char),
        Byte(u8),
    }

    impl Literal {
        pub fn is_unicode(&self) -> bool {
            match self.kind {
                LiteralKind::Unicode(_) => true,
                LiteralKind::Byte(b) if b <= 0x7F => true,
                LiteralKind::Byte(_) => false,
            }
        }
    }

    let literal_exactly_7f = Literal { kind: LiteralKind::Byte(0x7F) };
    assert_eq!(literal_exactly_7f.is_unicode(), true);
}

