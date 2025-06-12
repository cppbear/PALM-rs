// Answer 0

#[test]
fn test_is_f64_with_pos_int() {
    struct Number {
        n: N,
    }

    #[derive(Copy, Clone)]
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::PosInt(42) };
    assert_eq!(number.is_f64(), false);
}

#[test]
fn test_is_f64_with_neg_int() {
    struct Number {
        n: N,
    }

    #[derive(Copy, Clone)]
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::NegInt(-42) };
    assert_eq!(number.is_f64(), false);
}

#[test]
fn test_is_f64_with_float() {
    struct Number {
        n: N,
    }

    #[derive(Copy, Clone)]
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::Float(3.14) };
    assert_eq!(number.is_f64(), true);
}

