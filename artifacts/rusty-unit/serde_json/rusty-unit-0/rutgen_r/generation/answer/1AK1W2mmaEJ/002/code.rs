// Answer 0

#[test]
fn test_as_array_mut_with_non_empty_array() {
    use serde_json::{json, Value};

    let mut v = json!([1, 2, 3]);
    let array_mut = v.as_array_mut().unwrap();
    array_mut.push(4);
    assert_eq!(v, json!([1, 2, 3, 4]));
}

#[test]
fn test_as_array_mut_with_empty_array() {
    use serde_json::{json, Value};

    let mut v = json!([]);
    let array_mut = v.as_array_mut().unwrap();
    array_mut.push(1);
    assert_eq!(v, json!([1]));
}

#[test]
fn test_as_array_mut_with_nested_array() {
    use serde_json::{json, Value};

    let mut v = json!([json!([1, 2]), json!(["a", "b"])]);
    let array_mut = v.as_array_mut().unwrap();
    array_mut[0].as_array_mut().unwrap().clear();
    assert_eq!(v, json!([[], json!(["a", "b"])]));
}

#[test]
fn test_as_array_mut_with_non_array_value() {
    use serde_json::{json, Value};

    let mut v = json!({"key": "value"});
    assert!(v.as_array_mut().is_none());
}

