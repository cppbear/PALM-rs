// Answer 0

#[test]
fn test_serialize_i32_min_value() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let serializer_ref = MapKeySerializer { ser: &mut serializer };
    serializer_ref.serialize_i32(-2147483648);
}

#[test]
fn test_serialize_i32_zero() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let serializer_ref = MapKeySerializer { ser: &mut serializer };
    serializer_ref.serialize_i32(0);
}

#[test]
fn test_serialize_i32_max_value() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let serializer_ref = MapKeySerializer { ser: &mut serializer };
    serializer_ref.serialize_i32(2147483647);
}

#[test]
#[should_panic]
fn test_serialize_i32_write_i32_err() {
    let writer = Err(io::Error);
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let serializer_ref = MapKeySerializer { ser: &mut serializer };
    serializer_ref.serialize_i32(123);
}

#[test]
#[should_panic]
fn test_serialize_i32_begin_string_err() {
    let writer = Err(io::Error);
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    let serializer_ref = MapKeySerializer { ser: &mut serializer };
    serializer_ref.serialize_i32(-45);
}

