// Answer 0

#[test]
fn test_as_u64_pos_int() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::PosInt(42) };
    assert_eq!(number.as_u64(), Some(42));
}

#[test]
fn test_as_u64_neg_int() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::NegInt(-42) };
    assert_eq!(number.as_u64(), None);
}

#[test]
fn test_as_u64_float() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::Float(42.0) };
    assert_eq!(number.as_u64(), None);
}

