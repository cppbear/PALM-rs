// Answer 0

#[test]
fn test_as_null_with_value_null() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(null);

    assert_eq!(v.as_null(), Some(()));
}

#[test]
fn test_as_null_with_value_other() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(false);

    assert_eq!(v.as_null(), None);
}

#[test]
fn test_as_null_with_value_empty_object() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!({});

    assert_eq!(v.as_null(), None);
}

#[test]
fn test_as_null_with_value_string() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!("not null");

    assert_eq!(v.as_null(), None);
}

#[test]
fn test_as_null_with_value_number() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!(42);

    assert_eq!(v.as_null(), None);
}

#[test]
fn test_as_null_with_value_array() {
    use serde_json::json;
    use serde_json::Value;

    let v = json!([1, 2, 3]);

    assert_eq!(v.as_null(), None);
}

