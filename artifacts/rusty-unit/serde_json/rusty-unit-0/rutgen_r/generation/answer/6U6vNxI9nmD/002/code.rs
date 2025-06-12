// Answer 0

#[test]
fn test_as_u64_with_negative_integer() {
    struct Number {
        n: N,
    }

    enum N {
        NegInt(i64),
        PosInt(u64),
        Float(f64),
    }

    let number = Number { n: N::NegInt(-42) };
    assert_eq!(number.as_u64(), None);
}

#[test]
fn test_as_u64_with_negative_integer_zero() {
    struct Number {
        n: N,
    }

    enum N {
        NegInt(i64),
        PosInt(u64),
        Float(f64),
    }

    let number = Number { n: N::NegInt(0) };
    assert_eq!(number.as_u64(), None);
}

