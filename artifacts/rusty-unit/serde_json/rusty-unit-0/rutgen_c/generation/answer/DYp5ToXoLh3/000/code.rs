// Answer 0

#[test]
fn test_serialize_bool_true() {
    let serializer = Serializer;
    let result = serializer.serialize_bool(true).unwrap();
    assert_eq!(result, Value::Bool(true));
}

#[test]
fn test_serialize_bool_false() {
    let serializer = Serializer;
    let result = serializer.serialize_bool(false).unwrap();
    assert_eq!(result, Value::Bool(false));
}

