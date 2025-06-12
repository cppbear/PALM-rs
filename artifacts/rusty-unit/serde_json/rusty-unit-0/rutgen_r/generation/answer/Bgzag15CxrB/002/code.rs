// Answer 0

#[test]
fn test_is_f64_with_neg_int() {
    struct Number {
        n: N,
    }

    enum N {
        NegInt(i64),
        PosInt(u64),
        Float(f64),
    }

    let number = Number { n: N::NegInt(-123) };
    assert_eq!(number.is_f64(), false);
}

