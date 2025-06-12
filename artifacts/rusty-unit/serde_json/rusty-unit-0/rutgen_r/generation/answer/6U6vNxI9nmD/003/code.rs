// Answer 0

#[test]
fn test_as_u64_positive_integer() {
    struct Number {
        n: N,
    }
    
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::PosInt(12345) };
    assert_eq!(number.as_u64(), Some(12345));
}

#[test]
fn test_as_u64_large_positive_integer() {
    struct Number {
        n: N,
    }
    
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::PosInt(18446744073709551615) }; // Max value for u64
    assert_eq!(number.as_u64(), Some(18446744073709551615));
}

#[test]
fn test_as_u64_zero() {
    struct Number {
        n: N,
    }
    
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number { n: N::PosInt(0) };
    assert_eq!(number.as_u64(), Some(0));
}

