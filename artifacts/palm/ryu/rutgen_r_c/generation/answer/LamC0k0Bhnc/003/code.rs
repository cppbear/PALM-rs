// Answer 0

#[test]
fn test_format_nonfinite_infinity() {
    struct NonFiniteTest(f64);

    impl NonFiniteTest {
        fn to_bits(self) -> u64 {
            self.0.to_bits()
        }
    }

    let test_value = NonFiniteTest(f64::INFINITY);
    assert_eq!(test_value.format_nonfinite(), "inf");
}

#[test]
fn test_format_nonfinite_neg_infinity() {
    struct NonFiniteTest(f64);

    impl NonFiniteTest {
        fn to_bits(self) -> u64 {
            self.0.to_bits()
        }
    }

    let test_value = NonFiniteTest(f64::NEG_INFINITY);
    assert_eq!(test_value.format_nonfinite(), "-inf");
}

#[test]
fn test_format_nonfinite_nan() {
    struct NonFiniteTest(f64);

    impl NonFiniteTest {
        fn to_bits(self) -> u64 {
            self.0.to_bits()
        }
    }

    let test_value = NonFiniteTest(f64::NAN);
    assert_eq!(test_value.format_nonfinite(), "NaN");
}

