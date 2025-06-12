// Answer 0

#[test]
fn test_is_f64_with_pos_int() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let pos_int_value = 42u64; // example positive integer value
    let number = Number { n: N::PosInt(pos_int_value) };

    assert_eq!(number.is_f64(), false);
}

#[test]
fn test_is_f64_with_neg_int() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let neg_int_value = -42i64; // example negative integer value
    let number = Number { n: N::NegInt(neg_int_value) };

    assert_eq!(number.is_f64(), false);
}

#[test]
fn test_is_f64_with_float() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let float_value = 3.14f64; // example float value
    let number = Number { n: N::Float(float_value) };

    assert_eq!(number.is_f64(), true);
}

