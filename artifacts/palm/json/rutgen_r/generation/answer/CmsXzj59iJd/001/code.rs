// Answer 0

#[derive(Debug)]
struct Serializer;

impl Serializer {
    fn serialize_i64(self, value: i64) -> Result<Value> {
        // Hypothetical implementation for testing
        Ok(Value::Number(value.into()))
    }

    fn serialize_i32(self, value: i32) -> Result<Value> {
        self.serialize_i64(value as i64)
    }
}

#[derive(Debug)]
enum Value {
    Number(i64),
}

#[derive(Debug)]
struct Result<T>(Option<T>);

#[test]
fn test_serialize_i32_positive() {
    let serializer = Serializer;
    let result = serializer.serialize_i32(42);
    assert!(result.0.is_some());
    if let Some(value) = result.0 {
        match value {
            Value::Number(num) => assert_eq!(num, 42),
        }
    }
}

#[test]
fn test_serialize_i32_negative() {
    let serializer = Serializer;
    let result = serializer.serialize_i32(-42);
    assert!(result.0.is_some());
    if let Some(value) = result.0 {
        match value {
            Value::Number(num) => assert_eq!(num, -42),
        }
    }
}

#[test]
fn test_serialize_i32_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_i32(0);
    assert!(result.0.is_some());
    if let Some(value) = result.0 {
        match value {
            Value::Number(num) => assert_eq!(num, 0),
        }
    }
}

#[test]
fn test_serialize_i32_boundary() {
    let serializer = Serializer;
    let result = serializer.serialize_i32(i32::MAX);
    assert!(result.0.is_some());
    if let Some(value) = result.0 {
        match value {
            Value::Number(num) => assert_eq!(num, i32::MAX as i64),
        }
    }

    let result = serializer.serialize_i32(i32::MIN);
    assert!(result.0.is_some());
    if let Some(value) = result.0 {
        match value {
            Value::Number(num) => assert_eq!(num, i32::MIN as i64),
        }
    }
}

