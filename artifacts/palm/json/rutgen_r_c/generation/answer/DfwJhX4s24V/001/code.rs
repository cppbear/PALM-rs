// Answer 0

#[test]
fn test_serialize_u32_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_u32(0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(0.into()));
}

#[test]
fn test_serialize_u32_max() {
    let serializer = Serializer;
    let result = serializer.serialize_u32(u32::MAX);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(u32::MAX as u64.into()));
}

#[test]
fn test_serialize_u32_one() {
    let serializer = Serializer;
    let result = serializer.serialize_u32(1);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(1.into()));
}

#[test]
fn test_serialize_u32_large_number() {
    let serializer = Serializer;
    let value = 123456789u32;
    let result = serializer.serialize_u32(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(value as u64.into()));
}

