// Answer 0

#[test]
fn test_serialize_f64_infinity() {
    let result = serialize_f64(1.0 / 0.0); // Positive Infinity
    assert_eq!(result, Err(float_key_must_be_finite()));
}

#[test]
fn test_serialize_f64_negative_infinity() {
    let result = serialize_f64(-1.0 / 0.0); // Negative Infinity
    assert_eq!(result, Err(float_key_must_be_finite()));
}

#[test]
fn test_serialize_f64_nan() {
    let result = serialize_f64(0.0 / 0.0); // NaN
    assert_eq!(result, Err(float_key_must_be_finite()));
}

