// Answer 0

#[test]
fn test_serialize_f64_positive() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(3.14);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(Number::from(3.14)));
}

#[test]
fn test_serialize_f64_negative() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(-2.718);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(Number::from(-2.718)));
}

#[test]
fn test_serialize_f64_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(0.0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(Number::from(0.0)));
}

#[test]
fn test_serialize_f64_infinity() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(f64::INFINITY);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(Number::from(f64::INFINITY)));
}

#[test]
fn test_serialize_f64_neg_infinity() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(f64::NEG_INFINITY);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(Number::from(f64::NEG_INFINITY)));
}

#[test]
fn test_serialize_f64_nan() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(f64::NAN);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(Number::from(f64::NAN)));
}

