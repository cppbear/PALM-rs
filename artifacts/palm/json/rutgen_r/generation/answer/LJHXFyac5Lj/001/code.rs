// Answer 0

#[test]
fn test_as_i64_with_float() {
    struct Number {
        n: N,
    }

    enum N {
        Float(f64),
        PosInt(u64),
        NegInt(i64),
    }

    let number_float = Number { n: N::Float(3.14) };
    assert_eq!(number_float.as_i64(), None);
}

