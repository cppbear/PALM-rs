// Answer 0

#[test]
fn test_from_i128_within_i64_range() {
    struct Number {
        n: N,
    }

    enum N {
        NegInt(i64),
        PosInt(u64),
    }

    let result = from_i128(128);
    assert!(result.is_some());
    if let Some(Number { n }) = result {
        if let N::PosInt(value) = n {
            assert_eq!(value, 128u64);
        } else {
            panic!("Expected a PosInt");
        }
    }
}

#[test]
fn test_from_i128_negative_within_i64_range() {
    struct Number {
        n: N,
    }

    enum N {
        NegInt(i64),
        PosInt(u64),
    }

    let result = from_i128(-128);
    assert!(result.is_some());
    if let Some(Number { n }) = result {
        if let N::NegInt(value) = n {
            assert_eq!(value, -128i64);
        } else {
            panic!("Expected a NegInt");
        }
    }
}

#[test]
fn test_from_i128_exceeding_u64_max() {
    struct Number {
        n: N,
    }

    enum N {
        NegInt(i64),
        PosInt(u64),
    }

    let result = from_i128(1u128 << 64);
    assert!(result.is_none());
}

#[test]
fn test_from_i128_below_i64_min() {
    struct Number {
        n: N,
    }

    enum N {
        NegInt(i64),
        PosInt(u64),
    }

    let result = from_i128(i128::MIN);
    assert!(result.is_none());
}

