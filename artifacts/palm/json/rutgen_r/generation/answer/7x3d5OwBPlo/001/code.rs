// Answer 0

#[test]
fn test_as_number_with_non_number_value() {
    use serde_json::{json, Value};

    let v_string = json!("string");
    let v_boolean = json!(true);
    let v_array = json!([1, 2, 3]);
    let v_object = json!({"key": "value"});

    assert_eq!(v_string.as_number(), None);
    assert_eq!(v_boolean.as_number(), None);
    assert_eq!(v_array.as_number(), None);
    assert_eq!(v_object.as_number(), None);
}

#[test]
fn test_as_number_with_null_value() {
    use serde_json::{json, Value};

    let v_null = json!(null);

    assert_eq!(v_null.as_number(), None);
}

