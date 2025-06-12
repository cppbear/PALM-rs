// Answer 0

#[test]
fn test_serialize_seq_with_some_length() {
    let serializer = Serializer;
    let result = serializer.serialize_seq(Some(5)).unwrap();
    assert_eq!(result.vec.capacity(), 5);
}

#[test]
fn test_serialize_seq_with_none_length() {
    let serializer = Serializer;
    let result = serializer.serialize_seq(None).unwrap();
    assert_eq!(result.vec.capacity(), 0);
}

#[test]
fn test_serialize_seq_with_zero_length() {
    let serializer = Serializer;
    let result = serializer.serialize_seq(Some(0)).unwrap();
    assert_eq!(result.vec.capacity(), 0);
}

