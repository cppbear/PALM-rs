// Answer 0

#[test]
fn test_is_i64_float() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number_float = Number { n: N::Float(3.14) };
    assert_eq!(number_float.is_i64(), false);

    let number_float_neg = Number { n: N::Float(-2.718) };
    assert_eq!(number_float_neg.is_i64(), false);

    let number_float_large = Number { n: N::Float(1.0e10) };
    assert_eq!(number_float_large.is_i64(), false);

    let number_float_zero = Number { n: N::Float(0.0) };
    assert_eq!(number_float_zero.is_i64(), false);
}

