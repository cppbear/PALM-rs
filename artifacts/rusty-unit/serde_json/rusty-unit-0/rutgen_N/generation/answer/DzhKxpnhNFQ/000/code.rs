// Answer 0

#[test]
fn test_serialize_f64_finite() {
    let value: f64 = 3.14;
    let result = serialize_f64(value);
    assert_eq!(result, Ok("3.14".to_string()));
}

#[test]
#[should_panic]
fn test_serialize_f64_infinite() {
    let value: f64 = f64::INFINITY;
    let result = serialize_f64(value);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_serialize_f64_nan() {
    let value: f64 = f64::NAN;
    let result = serialize_f64(value);
    assert!(result.is_err());
}

