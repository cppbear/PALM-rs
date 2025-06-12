// Answer 0

#[test]
fn test_serialize_struct_variant_empty_name_zero_len() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_struct_variant("", 0, "", 0);
}

#[test]
fn test_serialize_struct_variant_empty_name_one_len() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_struct_variant("", 0, "test", 1);
}

#[test]
fn test_serialize_struct_variant_test_name_one_len() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_struct_variant("test", 1, "test", 2);
}

#[test]
fn test_serialize_struct_variant_test_name_max_index_max_len() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_struct_variant("test", u32::MAX, "test", usize::MAX);
}

#[test]
fn test_serialize_struct_variant_test_name_zero_len() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_struct_variant("test", 0, "test", 0);
}

#[test]
fn test_serialize_struct_variant_empty_name_max_index_max_len() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_struct_variant("", u32::MAX, "", usize::MAX);
}

