// Answer 0

#[test]
fn test_digits_unicode_long() {
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

    let hex_literal = HexLiteral {
        kind: HexLiteralKind::UnicodeLong,
    };

    let result = hex_literal.digits();
    assert_eq!(result, 8);
}

