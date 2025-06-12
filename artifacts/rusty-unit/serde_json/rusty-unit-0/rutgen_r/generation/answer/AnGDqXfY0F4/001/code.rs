// Answer 0

#[test]
fn test_from_str_valid_json() {
    let json_str = r#"{ "key": "value", "number": 42, "array": [1, 2, 3] }"#;
    let result: Result<Value, Error> = from_str(json_str);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value["key"], "value");
    assert_eq!(value["number"], 42);
    assert_eq!(value["array"], vec![1, 2, 3]);
}

#[test]
fn test_from_str_empty_string() {
    let json_str = r#"{}"#;
    let result: Result<Value, Error> = from_str(json_str);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert!(value.is_object());
}

#[test]
fn test_from_str_invalid_json() {
    let json_str = r#"{ "key": "value", "number": }"#;
    let result: Result<Value, Error> = from_str(json_str);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_from_str_panic_condition() {
    let json_str = r#"{ "key": "value", "number": "not_a_number" }"#;
    let result: Result<Value, Error> = from_str(json_str);
    let value = result.expect("Expected a valid JSON value");
    assert_eq!(value["number"].as_i64().unwrap(), 0);
}

