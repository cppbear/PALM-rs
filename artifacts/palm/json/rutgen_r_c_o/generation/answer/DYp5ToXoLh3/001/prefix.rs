// Answer 0

#[test]
fn test_serialize_bool_true() {
    let serializer = Serializer;
    let value = true;
    serializer.serialize_bool(value);
}

#[test]
fn test_serialize_bool_false() {
    let serializer = Serializer;
    let value = false;
    serializer.serialize_bool(value);
}

