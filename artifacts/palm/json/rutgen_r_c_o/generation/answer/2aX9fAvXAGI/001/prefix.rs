// Answer 0

#[test]
fn test_serialize_some_bool() {
    let serializer = MapKeySerializer;
    let value = &true;
    serializer.serialize_some(value);
}

#[test]
fn test_serialize_some_i8() {
    let serializer = MapKeySerializer;
    let value = &5i8;
    serializer.serialize_some(value);
}

#[test]
fn test_serialize_some_i16() {
    let serializer = MapKeySerializer;
    let value = &100i16;
    serializer.serialize_some(value);
}

#[test]
fn test_serialize_some_i32() {
    let serializer = MapKeySerializer;
    let value = &10000i32;
    serializer.serialize_some(value);
}

#[test]
fn test_serialize_some_i64() {
    let serializer = MapKeySerializer;
    let value = &1000000i64;
    serializer.serialize_some(value);
}

#[test]
fn test_serialize_some_u8() {
    let serializer = MapKeySerializer;
    let value = &10u8;
    serializer.serialize_some(value);
}

#[test]
fn test_serialize_some_u16() {
    let serializer = MapKeySerializer;
    let value = &200u16;
    serializer.serialize_some(value);
}

#[test]
fn test_serialize_some_u32() {
    let serializer = MapKeySerializer;
    let value = &200000u32;
    serializer.serialize_some(value);
}

#[test]
fn test_serialize_some_f32() {
    let serializer = MapKeySerializer;
    let value = &1.5f32;
    serializer.serialize_some(value);
}

#[test]
fn test_serialize_some_char() {
    let serializer = MapKeySerializer;
    let value = &'a';
    serializer.serialize_some(value);
}

#[test]
fn test_serialize_some_bytes() {
    let serializer = MapKeySerializer;
    let value = &b"some bytes"[..];
    serializer.serialize_some(value);
}

#[test]
fn test_serialize_some_vec() {
    let serializer = MapKeySerializer;
    let value = &vec![1, 2, 3];
    serializer.serialize_some(value);
}

