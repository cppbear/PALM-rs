// Answer 0

#[test]
fn test_serialize_tuple_struct_len_0() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    serializer.serialize_tuple_struct("test_struct", 0);
}

#[test]
fn test_serialize_tuple_struct_len_1() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    serializer.serialize_tuple_struct("test_struct", 1);
}

#[test]
fn test_serialize_tuple_struct_len_2() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    serializer.serialize_tuple_struct("test_struct", 2);
}

#[test]
fn test_serialize_tuple_struct_len_3() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    serializer.serialize_tuple_struct("test_struct", 3);
}

#[test]
fn test_serialize_tuple_struct_len_10() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    serializer.serialize_tuple_struct("test_struct", 10);
}

#[test]
fn test_serialize_tuple_struct_len_100() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    serializer.serialize_tuple_struct("test_struct", 100);
}

#[test]
fn test_serialize_tuple_struct_len_1000() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    serializer.serialize_tuple_struct("test_struct", 1000);
}

#[test]
fn test_serialize_tuple_struct_len_10_000() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    serializer.serialize_tuple_struct("test_struct", 10_000);
}

#[test]
fn test_serialize_tuple_struct_len_1_000_000() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    serializer.serialize_tuple_struct("test_struct", 1_000_000);
}

#[test]
#[should_panic]
fn test_serialize_tuple_struct_len_max_usize() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    serializer.serialize_tuple_struct("test_struct", std::usize::MAX);
}

