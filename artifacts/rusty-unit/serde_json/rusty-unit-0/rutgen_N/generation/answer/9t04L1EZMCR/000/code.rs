// Answer 0

#[test]
fn test_pointer_mut_root_empty_pointer() {
    use serde_json::Value;

    let mut value: Value = serde_json::from_str(r#"{"x": 1.0}"#).unwrap();
    let mutable_value = value.pointer_mut("").unwrap();
    assert_eq!(mutable_value, &mut value);
}

#[test]
fn test_pointer_mut_valid_pointer() {
    use serde_json::Value;

    let mut value: Value = serde_json::from_str(r#"{"x": 1.0}"#).unwrap();
    let mutable_ref = value.pointer_mut("/x").unwrap();
    *mutable_ref = 1.5.into();
    assert_eq!(value.pointer("/x"), Some(&1.5.into()));
}

#[test]
fn test_pointer_mut_nonexistent_pointer() {
    use serde_json::Value;

    let mut value: Value = serde_json::from_str(r#"{"x": 1.0}"#).unwrap();
    let mutable_ref = value.pointer_mut("/y");
    assert_eq!(mutable_ref, None);
}

#[test]
fn test_pointer_mut_nested_pointer() {
    use serde_json::Value;

    let mut value: Value = serde_json::from_str(r#"{"a": {"b": 2.0}}"#).unwrap();
    let mutable_ref = value.pointer_mut("/a/b").unwrap();
    *mutable_ref = 2.5.into();
    assert_eq!(value.pointer("/a/b"), Some(&2.5.into()));
}

#[test]
fn test_pointer_mut_invalid_start_non_slash() {
    use serde_json::Value;

    let mut value: Value = serde_json::from_str(r#"{"x": 1.0}"#).unwrap();
    let mutable_ref = value.pointer_mut("x");
    assert_eq!(mutable_ref, None);
}

#[test]
fn test_pointer_mut_array_index() {
    use serde_json::Value;

    let mut value: Value = serde_json::from_str(r#"[1.0, 2.0, 3.0]"#).unwrap();
    let mutable_ref = value.pointer_mut("/1").unwrap();
    *mutable_ref = 2.5.into();
    assert_eq!(value.pointer("/1"), Some(&2.5.into()));
}

#[test]
fn test_pointer_mut_array_invalid_index() {
    use serde_json::Value;

    let mut value: Value = serde_json::from_str(r#"[1.0, 2.0, 3.0]"#).unwrap();
    let mutable_ref = value.pointer_mut("/3");
    assert_eq!(mutable_ref, None);
}

