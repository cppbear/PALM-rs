// Answer 0

#[test]
fn test_get_mut_map() {
    use serde_json::json;
    use serde_json::Value;

    let mut object = json!({ "A": 65, "B": 66, "C": 67 });
    let value = object.get_mut("A").unwrap();
    *value = json!(69);
    assert_eq!(object.get("A"), Some(&json!(69)));
}

#[test]
fn test_get_mut_array() {
    use serde_json::json;
    use serde_json::Value;

    let mut array = json!([ "A", "B", "C" ]);
    let value = array.get_mut(2).unwrap();
    *value = json!("D");
    assert_eq!(array.get(2), Some(&json!("D")));
}

#[test]
fn test_get_mut_nonexistent_key_map() {
    use serde_json::json;

    let mut object = json!({ "A": 65 });
    let value = object.get_mut("B");
    assert_eq!(value, None);
}

#[test]
fn test_get_mut_out_of_bounds_array() {
    use serde_json::json;

    let mut array = json!([ "A", "B", "C" ]);
    let value = array.get_mut(3);
    assert_eq!(value, None);
}

#[test]
fn test_get_mut_wrong_type() {
    use serde_json::json;

    let mut object = json!([ 1, 2, 3 ]);
    let value = object.get_mut("A");
    assert_eq!(value, None);
}

