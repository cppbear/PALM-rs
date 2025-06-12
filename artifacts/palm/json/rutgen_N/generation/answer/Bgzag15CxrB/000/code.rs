// Answer 0

#[test]
fn test_is_f64_float() {
    struct Number {
        n: N,
    }

    enum N {
        Float(f64),
        PosInt(u64),
        NegInt(i64),
    }

    let number = Number { n: N::Float(3.14) };
    assert!(number.is_f64());
}

#[test]
fn test_is_f64_positive_integer() {
    struct Number {
        n: N,
    }

    enum N {
        Float(f64),
        PosInt(u64),
        NegInt(i64),
    }

    let number = Number { n: N::PosInt(42) };
    assert!(!number.is_f64());
}

#[test]
fn test_is_f64_negative_integer() {
    struct Number {
        n: N,
    }

    enum N {
        Float(f64),
        PosInt(u64),
        NegInt(i64),
    }

    let number = Number { n: N::NegInt(-42) };
    assert!(!number.is_f64());
}

#[cfg(feature = "arbitrary_precision")]
#[test]
fn test_is_f64_arbitrary_precision_float() {
    struct Number {
        n: N,
    }

    enum N {
        Float(f64),
        PosInt(u64),
        NegInt(i64),
        Arbitrary(String),
    }

    let number = Number { n: N::Arbitrary("3.14".to_string()) };
    assert!(number.is_f64());
}

#[cfg(feature = "arbitrary_precision")]
#[test]
fn test_is_f64_arbitrary_precision_integer() {
    struct Number {
        n: N,
    }

    enum N {
        Float(f64),
        PosInt(u64),
        NegInt(i64),
        Arbitrary(String),
    }

    let number = Number { n: N::Arbitrary("42".to_string()) };
    assert!(!number.is_f64());
}

#[cfg(feature = "arbitrary_precision")]
#[test]
fn test_is_f64_arbitrary_precision_scientific() {
    struct Number {
        n: N,
    }

    enum N {
        Float(f64),
        PosInt(u64),
        NegInt(i64),
        Arbitrary(String),
    }

    let number = Number { n: N::Arbitrary("1e10".to_string()) };
    assert!(number.is_f64());
}

