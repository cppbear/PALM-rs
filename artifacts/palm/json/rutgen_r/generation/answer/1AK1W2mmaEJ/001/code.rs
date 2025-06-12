// Answer 0

#[test]
fn test_as_array_mut_with_non_array_value() {
    use serde_json::{json, Value};

    // Test case: Value is a Boolean
    let mut boolean_value = json!(true);
    assert_eq!(boolean_value.as_array_mut(), None);

    // Test case: Value is a string
    let mut string_value = json!("test");
    assert_eq!(string_value.as_array_mut(), None);

    // Test case: Value is an object
    let mut object_value = json!({ "key": "value" });
    assert_eq!(object_value.as_array_mut(), None);

    // Test case: Value is a number
    let mut number_value = json!(42);
    assert_eq!(number_value.as_array_mut(), None);

    // Test case: Value is null
    let mut null_value = json!(null);
    assert_eq!(null_value.as_array_mut(), None);
}

#[test]
fn test_as_array_mut_with_empty_array() {
    use serde_json::{json, Value};

    // Test case: Value is an empty array
    let mut empty_array_value = json!([]);
    assert_eq!(empty_array_value.as_array_mut(), Some(&mut vec![]));
}

