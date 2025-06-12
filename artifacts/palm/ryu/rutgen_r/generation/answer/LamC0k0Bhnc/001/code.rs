// Answer 0

#[test]
fn test_format_nonfinite_nan() {
    struct TestFloat {
        bits: u32,
    }

    impl TestFloat {
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

    let test_value = TestFloat { bits: 0x7fffffff }; // Example representation of NaN
    assert_eq!(test_value.format_nonfinite(), "NAN");
}

