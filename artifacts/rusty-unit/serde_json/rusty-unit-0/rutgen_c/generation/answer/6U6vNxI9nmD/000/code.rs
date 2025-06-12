// Answer 0

#[test]
fn test_as_u64_positive_integer() {
    let number = Number::from(42u64);
    assert_eq!(number.as_u64(), Some(42));
}

#[test]
fn test_as_u64_negative_integer() {
    let number = Number::from(-42i64);
    assert_eq!(number.as_u64(), None);
}

#[test]
fn test_as_u64_float() {
    let number = Number::from(42.0f64);
    assert_eq!(number.as_u64(), None);
}

#[cfg(feature = "arbitrary_precision")]
#[test]
fn test_as_u64_large_value_arbitrary_precision() {
    let number = Number::from_string_unchecked("12345678901234567890".to_owned());
    assert_eq!(number.as_u64(), Some(12345678901234567890u64));
}

#[cfg(feature = "arbitrary_precision")]
#[test]
fn test_as_u64_negative_value_arbitrary_precision() {
    let number = Number::from_string_unchecked("-12345678901234567890".to_owned());
    assert_eq!(number.as_u64(), None);
}

