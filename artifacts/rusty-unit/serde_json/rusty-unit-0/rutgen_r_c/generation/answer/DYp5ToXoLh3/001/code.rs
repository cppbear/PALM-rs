// Answer 0

#[test]
fn test_serialize_bool_true() {
    let serializer = crate::ser::Serializer;
    let result = serializer.serialize_bool(true);
    assert_eq!(result, Ok(crate::value::Value::Bool(true)));
}

#[test]
fn test_serialize_bool_false() {
    let serializer = crate::ser::Serializer;
    let result = serializer.serialize_bool(false);
    assert_eq!(result, Ok(crate::value::Value::Bool(false)));
}

