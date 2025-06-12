// Answer 0

#[test]
fn test_is_f64_with_float() {
    #[cfg(not(feature = "arbitrary_precision"))]
    {
        let number = Number { n: N::Float(3.14) };
        assert!(number.is_f64());
    }
}

#[test]
fn test_is_f64_with_positive_int() {
    #[cfg(not(feature = "arbitrary_precision"))]
    {
        let number = Number { n: N::PosInt(42) };
        assert!(!number.is_f64());
    }
}

#[test]
fn test_is_f64_with_negative_int() {
    #[cfg(not(feature = "arbitrary_precision"))]
    {
        let number = Number { n: N::NegInt(-42) };
        assert!(!number.is_f64());
    }
}

#[test]
fn test_is_f64_with_arbitrary_precision_float() {
    #[cfg(feature = "arbitrary_precision")]
    {
        let number = Number::from_string_unchecked("3.14".to_owned());
        assert!(number.is_f64());
    }
}

#[test]
fn test_is_f64_with_arbitrary_precision_positive_int() {
    #[cfg(feature = "arbitrary_precision")]
    {
        let number = Number::from_string_unchecked("42".to_owned());
        assert!(!number.is_f64());
    }
}

#[test]
fn test_is_f64_with_arbitrary_precision_negative_int() {
    #[cfg(feature = "arbitrary_precision")]
    {
        let number = Number::from_string_unchecked("-42".to_owned());
        assert!(!number.is_f64());
    }
}

#[test]
fn test_is_f64_with_arbitrary_precision_float_not_finite() {
    #[cfg(feature = "arbitrary_precision")]
    {
        let number = Number::from_string_unchecked("NaN".to_owned());
        assert!(!number.is_f64());
    }
}

