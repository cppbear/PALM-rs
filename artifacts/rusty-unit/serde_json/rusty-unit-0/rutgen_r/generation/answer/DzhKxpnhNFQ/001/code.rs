// Answer 0

#[test]
fn test_serialize_f64_finite() {
    let value: f64 = 42.0;
    let result = serialize_f64(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ryu::Buffer::new().format_finite(value).to_owned());
}

#[test]
fn test_serialize_f64_negative_finite() {
    let value: f64 = -3.14;
    let result = serialize_f64(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ryu::Buffer::new().format_finite(value).to_owned());
}

#[test]
fn test_serialize_f64_zero() {
    let value: f64 = 0.0;
    let result = serialize_f64(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ryu::Buffer::new().format_finite(value).to_owned());
}

#[test]
fn test_serialize_f64_small_number() {
    let value: f64 = 1e-10;
    let result = serialize_f64(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ryu::Buffer::new().format_finite(value).to_owned());
}

#[test]
fn test_serialize_f64_large_number() {
    let value: f64 = 1e+10;
    let result = serialize_f64(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ryu::Buffer::new().format_finite(value).to_owned());
}

