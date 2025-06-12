// Answer 0

#[test]
fn test_as_f64_pos_int() {
    struct NWrapper {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let positive_int = NWrapper { n: N::PosInt(42) };
    let result = positive_int.as_f64();
    assert_eq!(result, Some(42.0));
}

#[test]
fn test_as_f64_neg_int() {
    struct NWrapper {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let negative_int = NWrapper { n: N::NegInt(-42) };
    let result = negative_int.as_f64();
    assert_eq!(result, Some(-42.0));
}

#[test]
fn test_as_f64_float() {
    struct NWrapper {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let float_value = NWrapper { n: N::Float(3.14) };
    let result = float_value.as_f64();
    assert_eq!(result, Some(3.14));
}

