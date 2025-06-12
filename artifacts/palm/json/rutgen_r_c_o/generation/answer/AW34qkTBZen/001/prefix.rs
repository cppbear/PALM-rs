// Answer 0

#[test]
fn test_serialize_bool_true() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_bool(true);
}

#[test]
fn test_serialize_bool_false() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_bool(false);
}

