// Answer 0

#[test]
fn test_from_str_valid_null() {
    let input = "null";
    let result: Result<Value, Error> = from_str(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Null);
}

#[test]
fn test_from_str_valid_bool() {
    let input = "true";
    let result: Result<Value, Error> = from_str(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Bool(true));

    let input = "false";
    let result: Result<Value, Error> = from_str(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Bool(false));
}

#[test]
fn test_from_str_valid_number() {
    let input = "12.5";
    let result: Result<Value, Error> = from_str(input);
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), Value::Number(_)));

    let input = "42";
    let result: Result<Value, Error> = from_str(input);
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), Value::Number(_)));
}

#[test]
fn test_from_str_valid_string() {
    let input = "\"a string\"";
    let result: Result<Value, Error> = from_str(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::String("a string".to_owned()));
}

#[test]
fn test_from_str_valid_array() {
    let input = "[\"an\", \"array\"]";
    let result: Result<Value, Error> = from_str(input);
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), Value::Array(_)));
}

#[test]
fn test_from_str_valid_object() {
    let input = "{\"an\": \"object\"}";
    let result: Result<Value, Error> = from_str(input);
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), Value::Object(_)));
}

#[test]
fn test_from_str_invalid_input() {
    let input = "invalid json";
    let result: Result<Value, Error> = from_str(input);
    assert!(result.is_err());
}

#[test]
fn test_from_str_empty_string() {
    let input = "";
    let result: Result<Value, Error> = from_str(input);
    assert!(result.is_err());
}

