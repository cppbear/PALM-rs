// Answer 0

#[test]
fn test_format_nonfinite_negative_infinity() {
    struct FloatWrapper {
        bits: u32,
    }

    impl FloatWrapper {
        fn to_bits(self) -> u32 {
            self.bits
        }

        fn format_nonfinite(self) -> &'static str {
            const MANTISSA_MASK: u32 = 0x007fffff;
            const SIGN_MASK: u32 = 0x80000000;
            let bits = self.to_bits();
            if bits & MANTISSA_MASK != 0 {
                "NAN"
            } else if bits & SIGN_MASK != 0 {
                "NEG_INFINITY"
            } else {
                "INFINITY"
            }
        }
    }

    let negative_infinity = FloatWrapper { bits: 0x80000000 };
    assert_eq!(negative_infinity.format_nonfinite(), "NEG_INFINITY");
}

