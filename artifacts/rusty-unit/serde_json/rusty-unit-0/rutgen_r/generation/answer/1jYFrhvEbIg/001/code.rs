// Answer 0

#[test]
fn test_get_mut_map_with_existing_key() {
    let mut object = serde_json::json!({ "A": 65, "B": 66, "C": 67 });
    let value = object.get_mut("A");
    assert!(value.is_some());
    *value.unwrap() = serde_json::json!(69);
    assert_eq!(object.get("A"), Some(&serde_json::json!(69)));
}

#[test]
fn test_get_mut_map_with_nonexistent_key() {
    let mut object = serde_json::json!({ "A": 65, "B": 66, "C": 67 });
    let value = object.get_mut("D");
    assert!(value.is_none());
}

#[test]
fn test_get_mut_array_with_valid_index() {
    let mut array = serde_json::json!([ "A", "B", "C" ]);
    let value = array.get_mut(1);
    assert!(value.is_some());
    *value.unwrap() = serde_json::json!("Z");
    assert_eq!(array.get(1), Some(&serde_json::json!("Z")));
}

#[test]
fn test_get_mut_array_with_out_of_bounds_index() {
    let mut array = serde_json::json!([ "A", "B", "C" ]);
    let value = array.get_mut(5);
    assert!(value.is_none());
}

#[test]
fn test_get_mut_number_instead_of_array() {
    let mut number = serde_json::json!(42);
    let value = number.get_mut(0);
    assert!(value.is_none());
}

#[test]
fn test_get_mut_on_empty_array() {
    let mut empty_array = serde_json::json!([]);
    let value = empty_array.get_mut(0);
    assert!(value.is_none());
}

