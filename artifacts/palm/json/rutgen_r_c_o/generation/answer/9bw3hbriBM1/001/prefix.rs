// Answer 0

#[test]
fn test_serialize_tuple_zero_length() {
    let serializer = MapKeySerializer { ser: &mut Serializer { writer: Vec::new(), formatter: Default::default() } };
    serializer.serialize_tuple(0);
}

#[test]
fn test_serialize_tuple_non_zero_length() {
    let serializer = MapKeySerializer { ser: &mut Serializer { writer: Vec::new(), formatter: Default::default() } };
    serializer.serialize_tuple(1);
}

