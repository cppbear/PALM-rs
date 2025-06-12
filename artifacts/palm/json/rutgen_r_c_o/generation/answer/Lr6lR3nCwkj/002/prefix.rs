// Answer 0

#[test]
fn test_from_f64_nan() {
    let result = Number::from_f64(f64::NAN);
}

#[test]
fn test_from_f64_infinity() {
    let result = Number::from_f64(f64::INFINITY);
}

#[test]
fn test_from_f64_neg_infinity() {
    let result = Number::from_f64(f64::NEG_INFINITY);
}

