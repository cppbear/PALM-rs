// Answer 0

#[test]
fn test_serialize_tuple_zero_length() {
    let writer = Vec::new();
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_tuple(0);
}

#[test]
fn test_serialize_tuple_one_length() {
    let writer = Vec::new();
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_tuple(1);
}

#[test]
fn test_serialize_tuple_two_length() {
    let writer = Vec::new();
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_tuple(2);
}

#[test]
fn test_serialize_tuple_ten_length() {
    let writer = Vec::new();
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_tuple(10);
}

#[test]
fn test_serialize_tuple_hundred_length() {
    let writer = Vec::new();
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_tuple(100);
}

#[test]
fn test_serialize_tuple_max_length() {
    let writer = Vec::new();
    let formatter = CompactFormatter {};
    let mut serializer = Serializer { writer, formatter };
    let _ = serializer.serialize_tuple(usize::MAX);
}

