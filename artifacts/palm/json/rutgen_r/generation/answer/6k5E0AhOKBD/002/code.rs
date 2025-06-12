// Answer 0

#[test]
fn test_is_u64_with_negative_integer() {
    struct Number {
        n: N,
    }
    
    enum N {
        NegInt(i64),
        PosInt(u64),
        Float(f64),
    }

    let negative_int = Number { n: N::NegInt(-1) };
    
    assert_eq!(negative_int.is_u64(), false);
}

#[test]
fn test_is_u64_with_float() {
    struct Number {
        n: N,
    }
    
    enum N {
        NegInt(i64),
        PosInt(u64),
        Float(f64),
    }

    let float_number = Number { n: N::Float(3.14) };
    
    assert_eq!(float_number.is_u64(), false);
}

