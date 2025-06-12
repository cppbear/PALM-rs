// Answer 0

#[test]
fn test_as_object_mut_non_object_case() {
    use serde_json::{Value, Map};

    // Case 1: Value is a String
    let mut v_string = Value::String(String::from("not an object"));
    assert_eq!(v_string.as_object_mut(), None);

    // Case 2: Value is a Number
    let mut v_number = Value::Number(serde_json::Number::from(42));
    assert_eq!(v_number.as_object_mut(), None);

    // Case 3: Value is a Boolean
    let mut v_boolean = Value::Bool(true);
    assert_eq!(v_boolean.as_object_mut(), None);

    // Case 4: Value is an Array
    let mut v_array = Value::Array(vec![Value::Number(serde_json::Number::from(1)), Value::Number(serde_json::Number::from(2))]);
    assert_eq!(v_array.as_object_mut(), None);

    // Case 5: Value is Null
    let mut v_null = Value::Null;
    assert_eq!(v_null.as_object_mut(), None);
}

