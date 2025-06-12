// Answer 0

#[test]
fn test_u32_deserializer_into_deserializer_min() {
    let deserializer = U32Deserializer::new(0);
    let _ = deserializer.into_deserializer();
}

#[test]
fn test_u32_deserializer_into_deserializer_mid() {
    let deserializer = U32Deserializer::new(2147483648);
    let _ = deserializer.into_deserializer();
}

#[test]
fn test_u32_deserializer_into_deserializer_max() {
    let deserializer = U32Deserializer::new(4294967295);
    let _ = deserializer.into_deserializer();
}

#[test]
fn test_u32_deserializer_into_deserializer_random() {
    let deserializer = U32Deserializer::new(123456);
    let _ = deserializer.into_deserializer();
}

