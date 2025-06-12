// Answer 0

#[test]
fn test_as_i64_positive_integer_within_limit() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::PosInt(42) };
    assert_eq!(number.as_i64(), Some(42));
}

#[test]
fn test_as_i64_positive_integer_exceeds_limit() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::PosInt(u64::MAX) }; // Exceeds i64::MAX
    assert_eq!(number.as_i64(), None);
}

#[test]
fn test_as_i64_negative_integer() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::NegInt(-10) };
    assert_eq!(number.as_i64(), Some(-10));
}

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

