// Answer 0

#[test]
fn test_as_i64_float() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::Float(3.14) };
    assert_eq!(number.as_i64(), None);
}

#[test]
fn test_as_i64_negative_float() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::Float(-2.71) };
    assert_eq!(number.as_i64(), None);
}

#[test]
fn test_as_i64_zero_float() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::Float(0.0) };
    assert_eq!(number.as_i64(), None);
}

