// Answer 0

#[test]
fn test_to_vec_with_string() {
    let value = &String::from("test");
    let result = to_vec(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"\"test\"".to_vec());
}

#[test]
fn test_to_vec_with_integer() {
    let value = &42;
    let result = to_vec(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"42".to_vec());
}

#[test]
fn test_to_vec_with_float() {
    let value = &3.14;
    let result = to_vec(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"3.14".to_vec());
}

#[test]
fn test_to_vec_with_array() {
    let value = &[1, 2, 3];
    let result = to_vec(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"[1,2,3]".to_vec());
}

#[test]
fn test_to_vec_with_empty_string() {
    let value = &String::from("");
    let result = to_vec(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"\"\"".to_vec());
}

#[test]
fn test_to_vec_with_nested_structures() {
    #[derive(Serialize)]
    struct Nested {
        name: String,
        value: i32,
    }
    
    let value = &Nested {
        name: String::from("nested"),
        value: 10,
    };
    
    let result = to_vec(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"{\"name\":\"nested\",\"value\":10}".to_vec());
}

#[test]
fn test_to_vec_with_map() {
    use serde_json::json;

    let value = &json!({
        "key": "value",
        "number": 123,
        "array": [1, 2, 3]
    });
    
    let result = to_vec(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"{\"key\":\"value\",\"number\":123,\"array\":[1,2,3]}".to_vec());
}

