// Answer 0

#[test]
fn test_serialize_tuple_len_0() {
    let serializer = Serializer;
    let _ = serializer.serialize_tuple(0);
}

#[test]
fn test_serialize_tuple_len_1() {
    let serializer = Serializer;
    let _ = serializer.serialize_tuple(1);
}

#[test]
fn test_serialize_tuple_len_10() {
    let serializer = Serializer;
    let _ = serializer.serialize_tuple(10);
}

#[test]
fn test_serialize_tuple_len_100() {
    let serializer = Serializer;
    let _ = serializer.serialize_tuple(100);
}

#[test]
fn test_serialize_tuple_len_1024() {
    let serializer = Serializer;
    let _ = serializer.serialize_tuple(1024);
}

#[test]
#[should_panic]
fn test_serialize_tuple_len_max() {
    let serializer = Serializer;
    let _ = serializer.serialize_tuple(usize::MAX);
}

