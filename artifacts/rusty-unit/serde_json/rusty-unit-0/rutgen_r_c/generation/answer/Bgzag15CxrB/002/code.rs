// Answer 0

#[test]
fn test_is_f64_with_negative_integer() {
    struct NegativeInteger(u64);
    
    impl From<i64> for Number {
        fn from(i: i64) -> Self {
            let n = if i < 0 { N::NegInt(i) } else { N::PosInt(i as u64) };
            Number { n }
        }
    }

    let negative_int = Number::from(-42i64);
    assert!(!negative_int.is_f64());
}

#[test]
fn test_is_f64_with_positive_integer() {
    struct PositiveInteger(u64);
    
    impl From<u64> for Number {
        fn from(u: u64) -> Self {
            let n = N::PosInt(u);
            Number { n }
        }
    }

    let positive_int = Number::from(42u64);
    assert!(!positive_int.is_f64());
}

#[test]
fn test_is_f64_with_float() {
    struct FloatNumber(f64);
    
    impl From<f64> for Number {
        fn from(f: f64) -> Self {
            let n = N::Float(f);
            Number { n }
        }
    }
    
    let float_number = Number::from(42.0f64);
    assert!(float_number.is_f64());
}

