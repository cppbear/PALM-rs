// Answer 0

#[test]
fn test_digits_unicode_long() {
    enum HexLiteralKind {
        X,
        UnicodeShort,
        UnicodeLong,
    }

    struct HexLiteral {
        kind: HexLiteralKind,
    }

    impl HexLiteral {
        pub fn digits(&self) -> u32 {
            match self.kind {
                HexLiteralKind::X => 2,
                HexLiteralKind::UnicodeShort => 4,
                HexLiteralKind::UnicodeLong => 8,
            }
        }
    }

    let hex_literal = HexLiteral { kind: HexLiteralKind::UnicodeLong };
    assert_eq!(hex_literal.digits(), 8);
}

