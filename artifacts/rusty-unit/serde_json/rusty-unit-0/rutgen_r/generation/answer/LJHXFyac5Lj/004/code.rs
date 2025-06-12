// Answer 0

#[test]
fn test_as_i64_pos_int_overflow() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let large_positive_number = Number {
        n: N::PosInt(i64::MAX as u64 + 1),
    };

    assert_eq!(large_positive_number.as_i64(), None);
}

#[test]
fn test_as_i64_neg_int() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let negative_number = Number {
        n: N::NegInt(-1),
    };

    assert_eq!(negative_number.as_i64(), Some(-1));
} 

#[test]
fn test_as_i64_float() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let float_number = Number {
        n: N::Float(1.5),
    };

    assert_eq!(float_number.as_i64(), None);
}

