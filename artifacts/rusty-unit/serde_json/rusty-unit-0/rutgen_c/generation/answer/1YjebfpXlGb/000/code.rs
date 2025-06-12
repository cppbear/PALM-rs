// Answer 0

#[test]
fn test_as_f32_positive_integer() {
    struct NumberPositive {
        n: N,
    }
    
    let number = NumberPositive { n: N::PosInt(10) };
    assert_eq!(number.as_f32(), Some(10.0));
}

#[test]
fn test_as_f32_negative_integer() {
    struct NumberNegative {
        n: N,
    }
    
    let number = NumberNegative { n: N::NegInt(-10) };
    assert_eq!(number.as_f32(), Some(-10.0));
}

#[test]
fn test_as_f32_float() {
    struct NumberFloat {
        n: N,
    }

    let number = NumberFloat { n: N::Float(10.5) };
    assert_eq!(number.as_f32(), Some(10.5));
}

#[test]
fn test_as_f32_arbitrary_precision_positive() {
    #[cfg(feature = "arbitrary_precision")]
    {
        struct NumberArbitrary {
            n: String,
        }

        let number = NumberArbitrary { n: "10.5".to_string() };
        assert_eq!(number.as_f32(), Some(10.5));
    }
}

#[test]
fn test_as_f32_arbitrary_precision_negative() {
    #[cfg(feature = "arbitrary_precision")]
    {
        struct NumberArbitrary {
            n: String,
        }

        let number = NumberArbitrary { n: "-10.5".to_string() };
        assert_eq!(number.as_f32(), Some(-10.5));
    }
}

#[test]
fn test_as_f32_arbitrary_precision_invalid() {
    #[cfg(feature = "arbitrary_precision")]
    {
        struct NumberArbitrary {
            n: String,
        }

        let number = NumberArbitrary { n: "invalid".to_string() };
        assert_eq!(number.as_f32(), None);
    }
}

