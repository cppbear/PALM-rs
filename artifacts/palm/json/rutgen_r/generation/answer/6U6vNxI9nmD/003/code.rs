// Answer 0

#[test]
fn test_as_u64_positive_integer() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number {
        n: N::PosInt(42),
    };
    assert_eq!(number.as_u64(), Some(42));
}

#[test]
fn test_as_u64_large_positive_integer() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number {
        n: N::PosInt(1_000_000_000_000),
    };
    assert_eq!(number.as_u64(), Some(1_000_000_000_000));
}

#[test]
fn test_as_u64_zero() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number {
        n: N::PosInt(0),
    };
    assert_eq!(number.as_u64(), Some(0));
}

