// Answer 0

#[test]
fn test_as_i128_neg_int() {
    struct Number {
        n: N,
    }

    enum N {
        NegInt(i128),
        PosInt(i128),
        Float(f64),
    }

    let number = Number { n: N::NegInt(-42) };
    assert_eq!(number.as_i128(), Some(-42));

    let number_large = Number { n: N::NegInt(i128::MIN) };
    assert_eq!(number_large.as_i128(), Some(i128::MIN));
}

#[test]
fn test_as_i128_pos_int() {
    struct Number {
        n: N,
    }

    enum N {
        NegInt(i128),
        PosInt(i128),
        Float(f64),
    }

    let number = Number { n: N::PosInt(100) };
    assert_eq!(number.as_i128(), Some(100));
}

#[test]
fn test_as_i128_float() {
    struct Number {
        n: N,
    }

    enum N {
        NegInt(i128),
        PosInt(i128),
        Float(f64),
    }

    let number = Number { n: N::Float(3.14) };
    assert_eq!(number.as_i128(), None);
}

