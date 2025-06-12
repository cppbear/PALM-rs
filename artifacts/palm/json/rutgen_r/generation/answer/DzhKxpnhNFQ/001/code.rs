// Answer 0

#[test]
fn test_serialize_f64_finite_positive() {
    let value: f64 = 123.456;
    let result = serialize_f64(value);
    assert_eq!(result, Ok(ryu::Buffer::new().format_finite(value).to_owned()));
}

#[test]
fn test_serialize_f64_finite_negative() {
    let value: f64 = -123.456;
    let result = serialize_f64(value);
    assert_eq!(result, Ok(ryu::Buffer::new().format_finite(value).to_owned()));
}

#[test]
fn test_serialize_f64_finite_zero() {
    let value: f64 = 0.0;
    let result = serialize_f64(value);
    assert_eq!(result, Ok(ryu::Buffer::new().format_finite(value).to_owned()));
}

#[test]
fn test_serialize_f64_finite_small() {
    let value: f64 = 1e-10;
    let result = serialize_f64(value);
    assert_eq!(result, Ok(ryu::Buffer::new().format_finite(value).to_owned()));
}

#[test]
fn test_serialize_f64_finite_large() {
    let value: f64 = 1e10;
    let result = serialize_f64(value);
    assert_eq!(result, Ok(ryu::Buffer::new().format_finite(value).to_owned()));
}

#[test]
fn test_serialize_f64_finite_precision() {
    let value: f64 = 1.123456789012345;
    let result = serialize_f64(value);
    assert_eq!(result, Ok(ryu::Buffer::new().format_finite(value).to_owned()));
}

