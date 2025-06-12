// Answer 0

#[derive(Debug)]
struct BytesDeserializer<'a> {
    value: &'a [u8],
    marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> BytesDeserializer<'a> {
    pub fn new(value: &'a [u8]) -> Self {
        BytesDeserializer {
            value,
            marker: std::marker::PhantomData,
        }
    }
}

#[test]
fn test_bytes_deserializer_with_non_empty_bytes() {
    let bytes = b"Hello, World!";
    let deserializer = BytesDeserializer::new(bytes);
    assert_eq!(deserializer.value, bytes);
}

#[test]
fn test_bytes_deserializer_with_empty_bytes() {
    let bytes: &[u8] = &[];
    let deserializer = BytesDeserializer::new(bytes);
    assert_eq!(deserializer.value, bytes);
}

#[test]
fn test_bytes_deserializer_with_large_data() {
    let bytes = vec![0u8; 1024 * 1024]; // 1MB of zeros
    let deserializer = BytesDeserializer::new(&bytes);
    assert_eq!(deserializer.value, &bytes);
}

