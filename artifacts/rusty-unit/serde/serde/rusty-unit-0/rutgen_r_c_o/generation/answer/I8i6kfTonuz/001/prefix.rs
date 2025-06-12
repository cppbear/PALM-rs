// Answer 0

#[test]
fn test_serialize_bool_true() {
    let mut map = /* initialize a suitable serialize map */;
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_bool(true);
}

#[test]
fn test_serialize_bool_false() {
    let mut map = /* initialize a suitable serialize map */;
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_bool(false);
}

