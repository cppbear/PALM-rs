// Answer 0

#[test]
fn test_as_i128_neg_int() {
    struct Number {
        n: N,
    }

    enum N {
        NegInt(i64),
        PosInt(i64),
        Float(f64),
    }

    let num_neg_int = Number { n: N::NegInt(-42) };
    assert_eq!(num_neg_int.as_i128(), Some(-42));

    let num_neg_int_bound = Number { n: N::NegInt(i64::MIN) };
    assert_eq!(num_neg_int_bound.as_i128(), Some(i128::from(i64::MIN)));

    let num_neg_int_zero = Number { n: N::NegInt(0) };
    assert_eq!(num_neg_int_zero.as_i128(), Some(0));
}

