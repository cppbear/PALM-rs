// Answer 0

#[test]
fn test_as_f64_positive_float() {
    let number = Number::from_f64(1.0).unwrap();
    let _ = number.as_f64();
}

#[test]
fn test_as_f64_negative_float() {
    let number = Number::from_f64(-1.0).unwrap();
    let _ = number.as_f64();
}

#[test]
fn test_as_f64_large_float() {
    let number = Number::from_f64(1.7976931348623157e+308).unwrap();
    let _ = number.as_f64();
}

#[test]
fn test_as_f64_small_float() {
    let number = Number::from_f64(-1.7976931348623157e+308).unwrap();
    let _ = number.as_f64();
}

#[test]
fn test_as_f64_zero() {
    let number = Number::from_f64(0.0).unwrap();
    let _ = number.as_f64();
}

#[test]
fn test_as_f64_positive_float_non_finite() {
    let number = Number::from_string_unchecked("Infinity".to_owned());
    let _ = number.as_f64();
}

#[test]
fn test_as_f64_negative_float_non_finite() {
    let number = Number::from_string_unchecked("-Infinity".to_owned());
    let _ = number.as_f64();
}

