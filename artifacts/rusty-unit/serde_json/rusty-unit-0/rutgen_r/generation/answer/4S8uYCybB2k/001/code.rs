// Answer 0

#[test]
fn test_as_u64_non_number() {
    use serde_json::{json, Value};

    let v_string = json!("string_value"); // Value that is a string
    let v_bool_true = json!(true); // Value that is a boolean true
    let v_bool_false = json!(false); // Value that is a boolean false
    let v_array = json!([1, 2, 3]); // Value that is an array
    let v_object = json!({"key": "value"}); // Value that is an object
    let v_null = json!(null); // Value that is null

    assert_eq!(v_string.as_u64(), None);
    assert_eq!(v_bool_true.as_u64(), None);
    assert_eq!(v_bool_false.as_u64(), None);
    assert_eq!(v_array.as_u64(), None);
    assert_eq!(v_object.as_u64(), None);
    assert_eq!(v_null.as_u64(), None);
}

