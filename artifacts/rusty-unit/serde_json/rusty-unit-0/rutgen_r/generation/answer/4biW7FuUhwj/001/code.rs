// Answer 0

#[test]
fn test_is_number_with_string() {
    use serde_json::json;
    let v = json!("2");
    assert!(!v.is_number());
}

#[test]
fn test_is_number_with_object() {
    use serde_json::json;
    let v = json!({ "key": "value" });
    assert!(!v.is_number());
}

#[test]
fn test_is_number_with_array() {
    use serde_json::json;
    let v = json!([1, 2, 3]);
    assert!(!v.is_number());
}

#[test]
fn test_is_number_with_boolean() {
    use serde_json::json;
    let v = json!(true);
    assert!(!v.is_number());
}

#[test]
fn test_is_number_with_null() {
    use serde_json::json;
    let v = json!(null);
    assert!(!v.is_number());
}

