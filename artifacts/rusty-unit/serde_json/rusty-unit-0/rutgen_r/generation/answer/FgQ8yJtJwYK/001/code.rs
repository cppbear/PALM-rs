// Answer 0

#[test]
fn test_serialize_f64_positive_float() {
    let float_value: f64 = 3.14;
    let result = serialize_f64(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

#[test]
fn test_serialize_f64_negative_float() {
    let float_value: f64 = -2.718;
    let result = serialize_f64(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

#[test]
fn test_serialize_f64_zero() {
    let float_value: f64 = 0.0;
    let result = serialize_f64(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

#[test]
fn test_serialize_f64_infinity() {
    let float_value: f64 = f64::INFINITY;
    let result = serialize_f64(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

#[test]
fn test_serialize_f64_negative_infinity() {
    let float_value: f64 = f64::NEG_INFINITY;
    let result = serialize_f64(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

#[test]
fn test_serialize_f64_nan() {
    let float_value: f64 = f64::NAN;
    let result = serialize_f64(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

