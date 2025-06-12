// Answer 0

#[test]
fn test_as_f64_positive_integer() {
    struct PositiveInteger(u64);

    impl From<u64> for Number {
        fn from(u: u64) -> Self {
            Number { n: N::PosInt(u) }
        }
    }

    let number = Number::from(42u64);
    assert_eq!(number.as_f64(), Some(42.0));
}

#[test]
fn test_as_f64_negative_integer() {
    struct NegativeInteger(i64);

    impl From<i64> for Number {
        fn from(i: i64) -> Self {
            Number { n: N::NegInt(i) }
        }
    }

    let number = Number::from(-42i64);
    assert_eq!(number.as_f64(), Some(-42.0));
}

#[test]
fn test_as_f64_float() {
    struct FloatValue(f64);

    impl From<f64> for Number {
        fn from(f: f64) -> Self {
            Number { n: N::Float(f) }
        }
    }

    let number = Number::from(3.14f64);
    assert_eq!(number.as_f64(), Some(3.14));
}

