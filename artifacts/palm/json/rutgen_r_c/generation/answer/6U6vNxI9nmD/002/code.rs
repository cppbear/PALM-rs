// Answer 0

#[test]
fn test_as_u64_with_neg_int() {
    struct NegativeIntNumber {
        n: N,
    }

    impl Number {
        fn new_neg_int(value: i64) -> Self {
            Number { n: N::NegInt(value) }
        }
    }

    let num = NegativeIntNumber { n: N::NegInt(-1) };
    assert_eq!(num.as_u64(), None);
}

#[test]
fn test_as_u64_with_float() {
    struct FloatNumber {
        n: N,
    }

    impl Number {
        fn new_float(value: f64) -> Self {
            Number { n: N::Float(value) }
        }
    }

    let num = FloatNumber { n: N::Float(1.23) };
    assert_eq!(num.as_u64(), None);
}

