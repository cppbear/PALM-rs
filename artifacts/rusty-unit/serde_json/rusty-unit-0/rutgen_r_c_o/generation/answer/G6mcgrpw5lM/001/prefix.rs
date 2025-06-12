// Answer 0

#[test]
fn test_serialize_newtype_struct_i8() {
    let mut serializer = Serializer::new();
    serializer.serialize_newtype_struct("value", &i8::MIN).unwrap();
    serializer.serialize_newtype_struct("value", &0i8).unwrap();
    serializer.serialize_newtype_struct("value", &i8::MAX).unwrap();
}

#[test]
fn test_serialize_newtype_struct_i16() {
    let mut serializer = Serializer::new();
    serializer.serialize_newtype_struct("value", &i16::MIN).unwrap();
    serializer.serialize_newtype_struct("value", &0i16).unwrap();
    serializer.serialize_newtype_struct("value", &i16::MAX).unwrap();
}

#[test]
fn test_serialize_newtype_struct_i32() {
    let mut serializer = Serializer::new();
    serializer.serialize_newtype_struct("value", &i32::MIN).unwrap();
    serializer.serialize_newtype_struct("value", &0i32).unwrap();
    serializer.serialize_newtype_struct("value", &i32::MAX).unwrap();
}

#[test]
fn test_serialize_newtype_struct_i64() {
    let mut serializer = Serializer::new();
    serializer.serialize_newtype_struct("value", &i64::MIN).unwrap();
    serializer.serialize_newtype_struct("value", &0i64).unwrap();
    serializer.serialize_newtype_struct("value", &i64::MAX).unwrap();
}

#[test]
fn test_serialize_newtype_struct_u8() {
    let mut serializer = Serializer::new();
    serializer.serialize_newtype_struct("value", &0u8).unwrap();
    serializer.serialize_newtype_struct("value", &u8::MAX).unwrap();
}

#[test]
fn test_serialize_newtype_struct_u16() {
    let mut serializer = Serializer::new();
    serializer.serialize_newtype_struct("value", &0u16).unwrap();
    serializer.serialize_newtype_struct("value", &u16::MAX).unwrap();
}

#[test]
fn test_serialize_newtype_struct_u32() {
    let mut serializer = Serializer::new();
    serializer.serialize_newtype_struct("value", &0u32).unwrap();
    serializer.serialize_newtype_struct("value", &u32::MAX).unwrap();
}

#[test]
fn test_serialize_newtype_struct_u64() {
    let mut serializer = Serializer::new();
    serializer.serialize_newtype_struct("value", &0u64).unwrap();
    serializer.serialize_newtype_struct("value", &u64::MAX).unwrap();
}

#[test]
fn test_serialize_newtype_struct_f32() {
    let mut serializer = Serializer::new();
    serializer.serialize_newtype_struct("value", &f32::MIN).unwrap();
    serializer.serialize_newtype_struct("value", &0.0f32).unwrap();
    serializer.serialize_newtype_struct("value", &f32::MAX).unwrap();
}

#[test]
fn test_serialize_newtype_struct_f64() {
    let mut serializer = Serializer::new();
    serializer.serialize_newtype_struct("value", &f64::MIN).unwrap();
    serializer.serialize_newtype_struct("value", &0.0f64).unwrap();
    serializer.serialize_newtype_struct("value", &f64::MAX).unwrap();
}

#[test]
fn test_serialize_newtype_struct_bool() {
    let mut serializer = Serializer::new();
    serializer.serialize_newtype_struct("value", &true).unwrap();
    serializer.serialize_newtype_struct("value", &false).unwrap();
}

#[test]
fn test_serialize_newtype_struct_char() {
    let mut serializer = Serializer::new();
    serializer.serialize_newtype_struct("value", &'\0').unwrap();
    serializer.serialize_newtype_struct("value", &'a').unwrap();
    serializer.serialize_newtype_struct("value", &'z').unwrap();
    serializer.serialize_newtype_struct("value", &'😊').unwrap();
}

#[test]
fn test_serialize_newtype_struct_str() {
    let mut serializer = Serializer::new();
    serializer.serialize_newtype_struct("value", "").unwrap();
    serializer.serialize_newtype_struct("value", "a").unwrap();
    serializer.serialize_newtype_struct("value", "long string...").unwrap();
    let long_string = "a".repeat(10000);
    serializer.serialize_newtype_struct("value", &long_string).unwrap();
}

#[test]
fn test_serialize_newtype_struct_bytes() {
    let mut serializer = Serializer::new();
    serializer.serialize_newtype_struct("value", &[]).unwrap();
    serializer.serialize_newtype_struct("value", &[1]).unwrap();
    serializer.serialize_newtype_struct("value", &[0, 1, 2, 3]).unwrap();
    let large_bytes = vec![0u8; 10000];
    serializer.serialize_newtype_struct("value", &large_bytes).unwrap();
}

