// Answer 0

#[test]
fn test_sort_all_objects_empty_object() {
    let mut value = serde_json::json!({});
    value.sort_all_objects();
    assert_eq!(value, serde_json::json!({}));
}

#[test]
fn test_sort_all_objects_single_key_object() {
    let mut value = serde_json::json!({"b": 1});
    value.sort_all_objects();
    assert_eq!(value, serde_json::json!({"b": 1}));
}

#[test]
fn test_sort_all_objects_multiple_key_object() {
    let mut value = serde_json::json!({"b": 1, "a": 2});
    value.sort_all_objects();
    assert_eq!(value, serde_json::json!({"a": 2, "b": 1}));
}

#[test]
fn test_sort_all_objects_nested_objects() {
    let mut value = serde_json::json!({
        "b": {"c": 3, "a": 1},
        "a": {"b": 2}
    });
    value.sort_all_objects();
    assert_eq!(value, serde_json::json!({
        "a": {"b": 2},
        "b": {"a": 1, "c": 3}
    }));
}

#[test]
fn test_sort_all_objects_array_of_objects() {
    let mut value = serde_json::json!([
        {"b": 1, "a": 2},
        {"c": 3, "a": 4}
    ]);
    value.sort_all_objects();
    assert_eq!(value, serde_json::json!([
        {"a": 2, "b": 1},
        {"a": 4, "c": 3}
    ]));
}

#[should_panic]
fn test_sort_all_objects_non_object_value() {
    let mut value = serde_json::json!(42);
    value.sort_all_objects();
}

#[test]
fn test_sort_all_objects_large_object() {
    let mut value = serde_json::json!({
        "z": 1,
        "y": 2,
        "x": 3,
        "w": {"a": 1, "b": 2},
        "v": {"c": 3}
    });
    value.sort_all_objects();
    assert_eq!(value, serde_json::json!({
        "v": {"c": 3},
        "w": {"a": 1, "b": 2},
        "x": 3,
        "y": 2,
        "z": 1
    }));
}

