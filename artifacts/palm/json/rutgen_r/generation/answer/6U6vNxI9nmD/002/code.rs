// Answer 0

#[test]
fn test_as_u64_neg_int() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::NegInt(-1) };
    assert_eq!(number.as_u64(), None);
}

