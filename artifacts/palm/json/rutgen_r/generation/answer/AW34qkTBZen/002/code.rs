// Answer 0

#[test]
fn test_serialize_bool_false() {
    let value: bool = false;
    let result = serialize_bool(value);
    assert_eq!(result, Ok("false".to_owned()));
}

#[test]
fn test_serialize_bool_true() {
    let value: bool = true;
    let result = serialize_bool(value);
    assert_eq!(result, Ok("true".to_owned()));
}

