// Answer 0

#[test]
fn test_as_f64_with_positive_integer() {
    struct PosInt {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = PosInt { n: N::PosInt(42) };
    assert_eq!(number.as_f64(), Some(42.0));
}

#[test]
fn test_as_f64_with_negative_integer() {
    struct NegInt {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = NegInt { n: N::NegInt(-42) };
    assert_eq!(number.as_f64(), Some(-42.0));
}

#[test]
fn test_as_f64_with_float() {
    struct Float {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Float { n: N::Float(3.14) };
    assert_eq!(number.as_f64(), Some(3.14));
}

#[test]
fn test_as_f64_with_arbitrary_precision() {
    #[cfg(feature = "arbitrary_precision")]
    {
        struct ArbitraryPrecision {
            n: N,
        }

        enum N {
            PosInt(u64),
            NegInt(i64),
            Float(f64),
        }

        let number = ArbitraryPrecision { n: N::Float(1e20) };
        assert_eq!(number.as_f64(), Some(1e20));
    }
}

#[test]
fn test_as_f64_with_finite_float() {
    struct FiniteFloat {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = FiniteFloat { n: N::Float(1.0) };
    assert_eq!(number.as_f64(), Some(1.0));
}

#[test]
fn test_as_f64_with_non_finite_float() {
    struct NonFiniteFloat {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = NonFiniteFloat { n: N::Float(f64::NAN) };
    assert_eq!(number.as_f64(), None);
}

