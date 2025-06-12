// Answer 0

#[test]
fn test_is_f64_with_positive_integer() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let num = Number { n: N::PosInt(42) };
    assert_eq!(num.is_f64(), false);
}

#[test]
fn test_is_f64_with_zero_positive_integer() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let num = Number { n: N::PosInt(0) };
    assert_eq!(num.is_f64(), false);
}

#[test]
fn test_is_f64_with_large_positive_integer() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let num = Number { n: N::PosInt(1_000_000) };
    assert_eq!(num.is_f64(), false);
}

