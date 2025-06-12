// Answer 0

#[test]
fn test_bytes_deserializer_null() {
    let value: &'static [u8] = &[];
    let _deserializer = BytesDeserializer::new(value);
}

#[test]
fn test_bytes_deserializer_empty() {
    let value: &[u8] = &[];
    let _deserializer = BytesDeserializer::new(value);
}

#[test]
fn test_bytes_deserializer_single_byte() {
    let value: &[u8] = &[0];
    let _deserializer = BytesDeserializer::new(value);
}

#[test]
fn test_bytes_deserializer_multi_byte() {
    let value: &[u8] = &[0, 1, 2, 3, 4, 255];
    let _deserializer = BytesDeserializer::new(value);
}

