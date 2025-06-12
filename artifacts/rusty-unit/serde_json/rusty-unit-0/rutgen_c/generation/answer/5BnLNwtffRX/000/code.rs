// Answer 0

#[test]
fn test_serialize_u64() {
    use crate::value::Value;
    use crate::error::Result;
    let serializer = Serializer;

    // Test a small value
    let result: Result<Value> = serializer.serialize_u64(42);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(42.into()));

    // Test a larger value
    let result: Result<Value> = serializer.serialize_u64(1_000_000);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(1_000_000.into()));

    // Test maximum u64 value
    let result: Result<Value> = serializer.serialize_u64(u64::MAX);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(u64::MAX.into()));
}

