// Answer 0

#[test]
fn test_as_number_not_a_number() {
    use serde_json::{json, Value, Number};

    let v_string = json!("This is a string");
    let v_array = json!([1, 2, 3]);
    let v_object = json!({"key": "value"});
    let v_bool_true = json!(true);
    let v_bool_false = json!(false);
    let v_null = json!(null);

    assert_eq!(v_string.as_number(), None);
    assert_eq!(v_array.as_number(), None);
    assert_eq!(v_object.as_number(), None);
    assert_eq!(v_bool_true.as_number(), None);
    assert_eq!(v_bool_false.as_number(), None);
    assert_eq!(v_null.as_number(), None);
}

