// Answer 0

#[test]
fn test_as_f64_with_value_not_number() {
    use serde_json::Value;

    let v_string = Value::String(String::from("not a number"));
    let v_array = Value::Array(vec![Value::from(1), Value::from(2)]);
    let v_object = Value::Object(serde_json::Map::new());

    assert_eq!(v_string.as_f64(), None);
    assert_eq!(v_array.as_f64(), None);
    assert_eq!(v_object.as_f64(), None);
}

#[test]
fn test_as_f64_with_value_bool() {
    use serde_json::Value;

    let v_true = Value::Bool(true);
    let v_false = Value::Bool(false);

    assert_eq!(v_true.as_f64(), None);
    assert_eq!(v_false.as_f64(), None);
}

