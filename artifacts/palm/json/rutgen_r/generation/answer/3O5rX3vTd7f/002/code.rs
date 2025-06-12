// Answer 0

#[test]
fn test_as_u128_neg_int() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u128),
        NegInt(i128),
        Float(f64),
    }

    let number = Number { n: N::NegInt(-42) };
    assert_eq!(number.as_u128(), None);
}

