// Answer 0

#[test]
fn test_is_i64_neg_int() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::NegInt(-1) };
    assert!(number.is_i64());
}

#[test]
fn test_is_i64_neg_int_boundary() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::NegInt(i64::MIN) };
    assert!(number.is_i64());
}

#[test]
fn test_is_i64_neg_int_large_negative() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::NegInt(-987654321) };
    assert!(number.is_i64());
}

