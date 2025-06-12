// Answer 0

#[test]
fn test_deserialize_bool() {
    let bool_json: &str = "true";
    let deserialized: Value = serde_json::from_str(bool_json).unwrap();
    assert_eq!(deserialized, Value::Bool(true));
}

#[test]
fn test_deserialize_i64() {
    let int_json: &str = "42";
    let deserialized: Value = serde_json::from_str(int_json).unwrap();
    assert_eq!(deserialized, Value::Number(Number::from(42)));
}

#[test]
fn test_deserialize_i128() {
    let big_int_json: &str = "170141183460469231731687303715884105727"; // i128 max value
    let deserialized: Value = serde_json::from_str(big_int_json).unwrap();
    assert_eq!(deserialized, Value::Number(Number::from_str(big_int_json).unwrap()));
}

#[test]
fn test_deserialize_u64() {
    let uint_json: &str = "100";
    let deserialized: Value = serde_json::from_str(uint_json).unwrap();
    assert_eq!(deserialized, Value::Number(Number::from(100)));
}

#[test]
fn test_deserialize_u128() {
    let big_uint_json: &str = "340282366920938463463374607431768211456"; // u128 max value
    let deserialized: Value = serde_json::from_str(big_uint_json).unwrap();
    assert_eq!(deserialized, Value::Number(Number::from_str(big_uint_json).unwrap()));
}

#[test]
fn test_deserialize_f64() {
    let float_json: &str = "12.34";
    let deserialized: Value = serde_json::from_str(float_json).unwrap();
    assert_eq!(deserialized, Value::Number(Number::from_f64(12.34).unwrap()));
}

#[test]
fn test_deserialize_string() {
    let string_json: &str = "\"Hello, World!\"";
    let deserialized: Value = serde_json::from_str(string_json).unwrap();
    assert_eq!(deserialized, Value::String("Hello, World!".to_string()));
}

#[test]
fn test_deserialize_null() {
    let null_json: &str = "null";
    let deserialized: Value = serde_json::from_str(null_json).unwrap();
    assert_eq!(deserialized, Value::Null);
}

#[test]
fn test_deserialize_empty_array() {
    let array_json: &str = "[]";
    let deserialized: Value = serde_json::from_str(array_json).unwrap();
    assert_eq!(deserialized, Value::Array(Vec::new()));
}

#[test]
fn test_deserialize_object() {
    let object_json: &str = "{\"key\": \"value\"}";
    let deserialized: Value = serde_json::from_str(object_json).unwrap();
    let mut expected = Map::new();
    expected.insert("key".to_string(), Value::String("value".to_string()));
    assert_eq!(deserialized, Value::Object(expected));
}

#[should_panic]
fn test_deserialize_invalid_key() {
    let invalid_object_json: &str = "{\"key\": \"value\", 42: \"number\"}";
    let _deserialized: Value = serde_json::from_str(invalid_object_json).unwrap();  // This should panic
}

