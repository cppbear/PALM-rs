// Answer 0

#[test]
fn test_from_i128_positive() {
    let result = Number::from_i128(128);
    assert!(result.is_some());
    if let Some(num) = result {
        assert_eq!(num.as_i128(), Some(128));
    }
}

#[test]
fn test_from_i128_negative() {
    let result = Number::from_i128(-128);
    assert!(result.is_some());
    if let Some(num) = result {
        assert_eq!(num.as_i128(), Some(-128));
    }
}

#[test]
fn test_from_i128_too_large() {
    #[cfg(not(feature = "arbitrary_precision"))]
    {
        let result = Number::from_i128(i128::MAX);
        assert!(result.is_none());
    }
}

#[test]
fn test_from_i128_too_small() {
    #[cfg(not(feature = "arbitrary_precision"))]
    {
        let result = Number::from_i128(i128::MIN);
        assert!(result.is_none());
    }
}

#[test]
fn test_from_i128_small_value() {
    let result = Number::from_i128(0);
    assert!(result.is_some());
    if let Some(num) = result {
        assert_eq!(num.as_i128(), Some(0));
    }
}

