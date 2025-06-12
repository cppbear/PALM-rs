// Answer 0

#[test]
fn test_is_i64_float_number() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let float_number = Number {
        n: N::Float(3.14),
    };

    assert_eq!(float_number.is_i64(), false);
}

#[test]
fn test_is_i64_negative_float_number() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let negative_float_number = Number {
        n: N::Float(-2.71),
    };

    assert_eq!(negative_float_number.is_i64(), false);
}

#[test]
fn test_is_i64_large_float_number() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let large_float_number = Number {
        n: N::Float(1e10),
    };

    assert_eq!(large_float_number.is_i64(), false);
}

