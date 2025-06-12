// Answer 0

#[test]
fn test_as_u64_with_float_number() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let float_number = Number { n: N::Float(3.14) };
    assert_eq!(float_number.as_u64(), None);
}

#[test]
fn test_as_u64_with_negative_integer() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let negative_integer = Number { n: N::NegInt(-5) };
    assert_eq!(negative_integer.as_u64(), None);
}

