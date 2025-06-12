// Answer 0

#[test]
fn test_serialize_tuple_struct_zero_length() {
    let serializer = Serializer;
    serializer.serialize_tuple_struct("test", 0);
}

#[test]
fn test_serialize_tuple_struct_one_length() {
    let serializer = Serializer;
    serializer.serialize_tuple_struct("test", 1);
}

#[test]
fn test_serialize_tuple_struct_two_length() {
    let serializer = Serializer;
    serializer.serialize_tuple_struct("test", 2);
}

#[test]
fn test_serialize_tuple_struct_ten_length() {
    let serializer = Serializer;
    serializer.serialize_tuple_struct("test", 10);
}

#[test]
#[should_panic]
fn test_serialize_tuple_struct_max_length() {
    let serializer = Serializer;
    serializer.serialize_tuple_struct("test", usize::MAX);
}

