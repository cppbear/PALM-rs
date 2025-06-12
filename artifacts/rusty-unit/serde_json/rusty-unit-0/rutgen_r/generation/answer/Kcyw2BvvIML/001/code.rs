// Answer 0

#[test]
fn test_to_value_with_struct() {
    use serde::Serialize;
    use serde_json::json;

    #[derive(Serialize)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let user = User {
        fingerprint: "0xF9BA143B95FF6D82".to_owned(),
        location: "Menlo Park, CA".to_owned(),
    };

    let expected = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA",
    });

    let result = serde_json::to_value(user).unwrap();
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn test_to_value_with_non_string_key_map() {
    use std::collections::BTreeMap;
    use serde_json::Value;

    let mut map = BTreeMap::new();
    map.insert(vec![32, 64], "x86");

    let _result: Result<Value, _> = serde_json::to_value(map);
}

#[test]
fn test_to_value_with_empty_struct() {
    use serde::Serialize;
    use serde_json::json;

    #[derive(Serialize)]
    struct Empty;

    let empty = Empty;

    let expected = json!(null);

    let result = serde_json::to_value(empty).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_to_value_with_string() {
    use serde_json::json;

    let value = "Test String";

    let expected = json!("Test String");

    let result = serde_json::to_value(value).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_to_value_with_integer() {
    use serde_json::json;

    let value = 42;

    let expected = json!(42);

    let result = serde_json::to_value(value).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_to_value_with_float() {
    use serde_json::json;

    let value = 3.14;

    let expected = json!(3.14);

    let result = serde_json::to_value(value).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_to_value_with_boolean() {
    use serde_json::json;

    let value = true;

    let expected = json!(true);

    let result = serde_json::to_value(value).unwrap();
    assert_eq!(result, expected);
}

