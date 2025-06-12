// Answer 0

#[test]
fn test_into_deserializer_valid_instance() {
    let bytes: &[u8] = &[1, 2, 3];
    let deserializer = BorrowedBytesDeserializer {
        value: bytes,
        marker: PhantomData,
    };
    let _ = deserializer.into_deserializer();
}

#[test]
fn test_into_deserializer_empty_instance() {
    let bytes: &[u8] = &[];
    let deserializer = BorrowedBytesDeserializer {
        value: bytes,
        marker: PhantomData,
    };
    let _ = deserializer.into_deserializer();
}

#[test]
fn test_into_deserializer_large_instance() {
    let bytes: Vec<u8> = (0..1_000_000).collect();
    let deserializer = BorrowedBytesDeserializer {
        value: &bytes,
        marker: PhantomData,
    };
    let _ = deserializer.into_deserializer();
}

#[test]
fn test_into_deserializer_single_byte_instance() {
    let bytes: &[u8] = &[42];
    let deserializer = BorrowedBytesDeserializer {
        value: bytes,
        marker: PhantomData,
    };
    let _ = deserializer.into_deserializer();
}

