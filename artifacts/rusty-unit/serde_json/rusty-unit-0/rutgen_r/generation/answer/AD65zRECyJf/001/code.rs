// Answer 0

#[test]
fn test_get_from_object_with_valid_key() {
    let object = json!({ "A": 65, "B": 66, "C": 67 });
    assert_eq!(*object.get("A").unwrap(), json!(65));
}

#[test]
fn test_get_from_array_with_valid_index() {
    let array = json!([ "A", "B", "C" ]);
    assert_eq!(*array.get(2).unwrap(), json!("C"));
}

#[test]
fn test_get_from_array_with_invalid_key() {
    let array = json!([ "A", "B", "C" ]);
    assert_eq!(array.get("A"), None);
}

#[test]
fn test_get_from_object_with_non_existent_key() {
    let object = json!({ "A": ["a", "á", "à"], "B": ["b", "b́"] });
    assert_eq!(object.get("D"), None);
}

#[test]
fn test_get_from_object_with_array_element() {
    let object = json!({
        "A": ["a", "á", "à"],
        "B": ["b", "b́"],
        "C": ["c", "ć", "ć̣", "ḉ"],
    });
    assert_eq!(object["B"][0], json!("b"));
}

#[test]
fn test_get_from_object_with_deep_indexing() {
    let object = json!({
        "A": { "x": { "y": { "z": "value" } } },
        "B": { "x": { "y": { "z": "other value" } } },
    });
    assert_eq!(object["A"]["x"]["y"]["z"], json!("value"));
    assert_eq!(object["B"]["x"]["y"]["z"], json!("other value"));
}

#[test]
fn test_get_with_invalid_deep_index() {
    let object = json!({
        "A": { "x": { "y": { "z": "value" } } },
        "B": { "x": { "y": { "z": "other value" } } },
    });
    assert_eq!(object["C"]["x"]["y"]["z"], json!(null));
}

#[test]
fn test_get_from_array_with_out_of_bounds_index() {
    let array = json!([ "A", "B", "C" ]);
    assert_eq!(array.get(5), None);
}

