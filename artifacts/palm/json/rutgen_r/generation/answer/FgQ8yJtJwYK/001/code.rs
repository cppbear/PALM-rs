// Answer 0

#[test]
fn test_serialize_f64_positive() {
    let float_value = 3.14;
    let result = serialize_f64(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

#[test]
fn test_serialize_f64_negative() {
    let float_value = -2.718;
    let result = serialize_f64(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

#[test]
fn test_serialize_f64_zero() {
    let float_value = 0.0;
    let result = serialize_f64(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

#[test]
fn test_serialize_f64_large() {
    let float_value = 1.7976931348623157e308; // Maximum f64 value
    let result = serialize_f64(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

#[test]
fn test_serialize_f64_small() {
    let float_value = 5e-324; // Minimum positive subnormal f64
    let result = serialize_f64(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

#[test]
fn test_serialize_f64_nan() {
    let float_value = std::f64::NAN; // Not-a-Number
    let result = serialize_f64(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

#[test]
fn test_serialize_f64_infinity() {
    let float_value = std::f64::INFINITY; // Positive Infinity
    let result = serialize_f64(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

#[test]
fn test_serialize_f64_neg_infinity() {
    let float_value = std::f64::NEG_INFINITY; // Negative Infinity
    let result = serialize_f64(float_value);
    assert_eq!(result, Ok(Value::from(float_value)));
}

