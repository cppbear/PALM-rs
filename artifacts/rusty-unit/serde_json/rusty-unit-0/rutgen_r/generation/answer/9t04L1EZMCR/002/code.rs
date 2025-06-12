// Answer 0

#[test]
fn test_pointer_mut_empty_pointer() {
    let mut value = serde_json::json!({"x": 1.0, "y": 2.0});
    assert_eq!(value.pointer_mut(""), Some(&mut value));
}

#[test]
fn test_pointer_mut_invalid_start() {
    let mut value = serde_json::json!({"x": 1.0, "y": 2.0});
    assert_eq!(value.pointer_mut("x/y"), None);
}

#[test]
fn test_pointer_mut_valid_object_key() {
    let mut value = serde_json::json!({"x": 1.0, "y": 2.0});
    let ptr = value.pointer_mut("/x").unwrap();
    *ptr = 1.5.into();
    assert_eq!(value.pointer("/x"), Some(&1.5.into()));
}

#[test]
fn test_pointer_mut_valid_object_key_steal() {
    let mut value = serde_json::json!({"x": 1.0});
    let old_x = value.pointer_mut("/x").map(serde_json::Value::take).unwrap();
    assert_eq!(old_x, 1.0);
    assert_eq!(value.pointer("/x").unwrap(), &serde_json::Value::Null);
}

#[test]
fn test_pointer_mut_valid_array_index() {
    let mut value = serde_json::json!([10, 20, 30]);
    if let Some(v) = value.pointer_mut("/1") {
        *v = 25.into();
    }
    assert_eq!(value.pointer("/1"), Some(&25.into()));
}

#[test]
fn test_pointer_mut_invalid_array_index() {
    let mut value = serde_json::json!([10, 20, 30]);
    assert_eq!(value.pointer_mut("/3"), None);
}

#[test]
fn test_pointer_mut_array_out_of_bounds() {
    let mut value = serde_json::json!([10, 20, 30]);
    assert_eq!(value.pointer_mut("/-1"), None);
}

