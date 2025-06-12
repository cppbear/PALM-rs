// Answer 0

#[test]
fn test_get_from_object_with_existing_key() {
    let object = serde_json::json!({ "A": 65, "B": 66, "C": 67 });
    assert_eq!(*object.get("A").unwrap(), serde_json::json!(65));
}

#[test]
fn test_get_from_array_with_existing_index() {
    let array = serde_json::json!([ "A", "B", "C" ]);
    assert_eq!(*array.get(2).unwrap(), serde_json::json!("C"));
}

#[test]
fn test_get_from_array_with_nonexistent_string_index() {
    let array = serde_json::json!([ "A", "B", "C" ]);
    assert_eq!(array.get("A"), None);
}

#[test]
fn test_get_from_object_with_nested_array() {
    let object = serde_json::json!({
        "A": ["a", "á", "à"],
        "B": ["b", "b́"],
        "C": ["c", "ć", "ć̣", "ḉ"],
    });
    assert_eq!(object["B"][0], serde_json::json!("b"));
}

#[test]
fn test_get_from_object_with_nonexistent_key() {
    let object = serde_json::json!({
        "A": ["a", "á", "à"],
        "B": ["b", "b́"],
        "C": ["c", "ć", "ć̣", "ḉ"],
    });
    assert_eq!(object["D"], serde_json::json!(null));
}

#[test]
fn test_get_from_nested_object_with_nonexistent_path() {
    let object = serde_json::json!([ "example" ]);
    assert_eq!(object[0]["x"]["y"]["z"], serde_json::json!(null));
}

