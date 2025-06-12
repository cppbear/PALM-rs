// Answer 0

#[test]
fn test_from_str_valid_json_null() {
    let result: Result<Value, Error> = from_str("null");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Null);
}

#[test]
fn test_from_str_valid_json_bool() {
    let result: Result<Value, Error> = from_str("true");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Bool(true));
}

#[test]
fn test_from_str_valid_json_number() {
    let result: Result<Value, Error> = from_str("42");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(Number { n: 42.0 }));
}

#[test]
fn test_from_str_valid_json_string() {
    let result: Result<Value, Error> = from_str("\"hello\"");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::String(String::from("hello")));
}

#[test]
fn test_from_str_valid_json_array() {
    let result: Result<Value, Error> = from_str("[1, 2, 3]");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Array(vec![
        Value::Number(Number { n: 1.0 }),
        Value::Number(Number { n: 2.0 }),
        Value::Number(Number { n: 3.0 }),
    ]));
}

#[test]
fn test_from_str_valid_json_object() {
    let result: Result<Value, Error> = from_str("{\"key\":\"value\"}");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Object(Map {
        map: vec![("key".to_string(), Value::String("value".to_string()))].into_iter().collect()
    }));
}

#[test]
fn test_from_str_invalid_json() {
    let result: Result<Value, Error> = from_str("invalid");
    assert!(result.is_err());
}

