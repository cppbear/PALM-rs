// Answer 0

#[test]
fn test_get_mut_map_with_existing_key() {
    let mut object = serde_json::json!({ "A": 65, "B": 66, "C": 67 });
    if let Some(value) = object.get_mut("A") {
        *value = serde_json::json!(69);
    }
    assert_eq!(object.get("A"), Some(&serde_json::json!(69)));
}

#[test]
fn test_get_mut_map_with_nonexistent_key() {
    let mut object = serde_json::json!({ "A": 65, "B": 66, "C": 67 });
    let value = object.get_mut("D");
    assert_eq!(value, None);
}

#[test]
fn test_get_mut_array_with_existing_index() {
    let mut array = serde_json::json!([ "A", "B", "C" ]);
    if let Some(value) = array.get_mut(2) {
        *value = serde_json::json!("D");
    }
    assert_eq!(array.get(2), Some(&serde_json::json!("D")));
}

#[test]
fn test_get_mut_array_with_out_of_bounds_index() {
    let mut array = serde_json::json!([ "A", "B", "C" ]);
    let value = array.get_mut(5);
    assert_eq!(value, None);
}

#[test]
fn test_get_mut_array_with_negative_index() {
    let mut array = serde_json::json!([ "A", "B", "C" ]);
    let value = array.get_mut(-1);
    assert_eq!(value, None);
}

#[test]
fn test_get_mut_number_with_string_index() {
    let mut number = serde_json::json!(42);
    let value = number.get_mut("not_a_valid_index");
    assert_eq!(value, None);
}

#[test]
fn test_get_mut_map_with_integer_index() {
    let mut object = serde_json::json!({ "A": 65, "B": 66 });
    let value = object.get_mut(1);
    assert_eq!(value, None);
}

