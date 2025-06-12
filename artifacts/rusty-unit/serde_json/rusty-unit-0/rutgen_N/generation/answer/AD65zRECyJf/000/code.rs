// Answer 0

#[test]
fn test_get_from_object_with_string_key() {
    let object = serde_json::json!({ "A": 65, "B": 66, "C": 67 });
    assert_eq!(*object.get("A").unwrap(), serde_json::json!(65));
    assert_eq!(object.get("D"), None);
}

#[test]
fn test_get_from_array_with_usize_index() {
    let array = serde_json::json!([ "A", "B", "C" ]);
    assert_eq!(*array.get(2).unwrap(), serde_json::json!("C"));
    assert_eq!(array.get(3), None);
}

#[test]
fn test_get_invalid_index() {
    let array = serde_json::json!([ "A", "B", "C" ]);
    assert_eq!(array.get("A"), None);
}

#[test]
fn test_square_brackets_object() {
    let object = serde_json::json!({
        "A": ["a", "á", "à"],
        "B": ["b", "b́"],
        "C": ["c", "ć", "ć̣", "ḉ"],
    });
    assert_eq!(object["B"][0], serde_json::json!("b"));
    assert_eq!(object["D"], serde_json::json!(null));
    assert_eq!(object[0]["x"]["y"]["z"], serde_json::json!(null));
}

#[test]
fn test_square_brackets_array() {
    let array = serde_json::json!([ "A", "B", "C" ]);
    assert_eq!(array[0], serde_json::json!("A"));
    assert_eq!(array[3], serde_json::json!(null));
}

