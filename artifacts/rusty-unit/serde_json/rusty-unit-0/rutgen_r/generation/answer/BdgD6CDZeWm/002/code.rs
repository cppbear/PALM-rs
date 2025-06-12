// Answer 0

#[test]
fn test_as_f64_with_positive_integer() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(i64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::PosInt(5) };
    assert_eq!(number.as_f64(), Some(5.0));
}

#[test]
fn test_as_f64_with_negative_integer() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(i64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::NegInt(-3) };
    assert_eq!(number.as_f64(), Some(-3.0));
}

#[test]
fn test_as_f64_with_float() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(i64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::Float(2.5) };
    assert_eq!(number.as_f64(), Some(2.5));
}

#[test]
#[should_panic]
fn test_as_f64_with_nan() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(i64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::Float(std::f64::NAN) };
    let _ = number.as_f64(); // Expecting panic
}

#[test]
#[should_panic]
fn test_as_f64_with_infinity() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(i64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::Float(std::f64::INFINITY) };
    let _ = number.as_f64(); // Expecting panic
}

