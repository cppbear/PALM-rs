// Answer 0

#[test]
fn test_as_i128_float_case() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::Float(3.14) };
    assert_eq!(number.as_i128(), None);
}

#[test]
fn test_as_i128_negative_float_case() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::Float(-2.71) };
    assert_eq!(number.as_i128(), None);
}

#[test]
fn test_as_i128_large_float_case() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::Float(1e10) };
    assert_eq!(number.as_i128(), None);
}

#[test]
fn test_as_i128_zero_float_case() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::Float(0.0) };
    assert_eq!(number.as_i128(), None);
}

