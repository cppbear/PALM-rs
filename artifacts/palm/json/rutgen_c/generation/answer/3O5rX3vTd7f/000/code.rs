// Answer 0

#[test]
fn test_as_u128_positive_integer() {
    let number = Number { n: N::PosInt(42) };
    assert_eq!(number.as_u128(), Some(42));
}

#[test]
fn test_as_u128_negative_integer() {
    let number = Number { n: N::NegInt(-1) };
    assert_eq!(number.as_u128(), None);
}

#[test]
fn test_as_u128_float() {
    let number = Number { n: N::Float(3.14) };
    assert_eq!(number.as_u128(), None);
}

#[cfg(feature = "arbitrary_precision")]
#[test]
fn test_as_u128_arbitrary_precision() {
    let number = Number::from_string_unchecked("12345678901234567890".to_owned());
    assert_eq!(number.as_u128(), Some(12345678901234567890));
}

#[cfg(feature = "arbitrary_precision")]
#[test]
fn test_as_u128_invalid_string() {
    let number = Number::from_string_unchecked("invalid".to_owned());
    assert_eq!(number.as_u128(), None);
}

