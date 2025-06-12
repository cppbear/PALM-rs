// Answer 0

#[test]
fn test_serialize_bool_true() {
    let value = true;
    let result = serialize_bool(value);
    assert_eq!(result, Ok(Value::Bool(true)));
}

#[test]
fn test_serialize_bool_false() {
    let value = false;
    let result = serialize_bool(value);
    assert_eq!(result, Ok(Value::Bool(false)));
}

