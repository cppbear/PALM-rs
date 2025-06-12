// Answer 0

#[test]
fn test_is_u64_neg_int() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let num = Number { n: N::NegInt(-1) };
    assert_eq!(num.is_u64(), false);
}

