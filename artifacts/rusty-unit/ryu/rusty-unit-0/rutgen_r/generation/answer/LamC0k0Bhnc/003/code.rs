// Answer 0

#[test]
fn test_format_nonfinite_infinity() {
    struct FloatWrapper {
        value: f32,
    }

    impl FloatWrapper {
        fn to_bits(self) -> u32 {
            self.value.to_bits()
        }

        fn format_nonfinite(self) -> &'static str {
            const MANTISSA_MASK: u32 = 0x007fffff;
            const SIGN_MASK: u32 = 0x80000000;
            let bits = self.to_bits();
            if bits & MANTISSA_MASK != 0 {
                "NaN"
            } else if bits & SIGN_MASK != 0 {
                "-Infinity"
            } else {
                "Infinity"
            }
        }
    }

    let positive_infinity = FloatWrapper { value: f32::INFINITY };
    assert_eq!(positive_infinity.format_nonfinite(), "Infinity");
}

#[test]
fn test_format_nonfinite_negative_infinity() {
    struct FloatWrapper {
        value: f32,
    }

    impl FloatWrapper {
        fn to_bits(self) -> u32 {
            self.value.to_bits()
        }

        fn format_nonfinite(self) -> &'static str {
            const MANTISSA_MASK: u32 = 0x007fffff;
            const SIGN_MASK: u32 = 0x80000000;
            let bits = self.to_bits();
            if bits & MANTISSA_MASK != 0 {
                "NaN"
            } else if bits & SIGN_MASK != 0 {
                "-Infinity"
            } else {
                "Infinity"
            }
        }
    }

    let negative_infinity = FloatWrapper { value: f32::NEG_INFINITY };
    assert_eq!(negative_infinity.format_nonfinite(), "-Infinity");
}

#[test]
fn test_format_nonfinite_nan() {
    struct FloatWrapper {
        value: f32,
    }

    impl FloatWrapper {
        fn to_bits(self) -> u32 {
            self.value.to_bits()
        }

        fn format_nonfinite(self) -> &'static str {
            const MANTISSA_MASK: u32 = 0x007fffff;
            const SIGN_MASK: u32 = 0x80000000;
            let bits = self.to_bits();
            if bits & MANTISSA_MASK != 0 {
                "NaN"
            } else if bits & SIGN_MASK != 0 {
                "-Infinity"
            } else {
                "Infinity"
            }
        }
    }

    let nan_value = FloatWrapper { value: f32::NAN };
    assert_eq!(nan_value.format_nonfinite(), "NaN");
}

