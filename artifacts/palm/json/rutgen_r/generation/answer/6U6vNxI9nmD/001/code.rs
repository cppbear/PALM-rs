// Answer 0

#[test]
fn test_as_u64_with_float() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    let number = Number {
        n: N::Float(3.14),
    };
    
    assert_eq!(number.as_u64(), None);
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

    let number = Number {
        n: N::NegInt(-5),
    };
    
    assert_eq!(number.as_u64(), None);
}

