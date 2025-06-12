// Answer 0

#[test]
fn test_new_borrowed_bytes_deserializer() {
    struct BorrowedBytesDeserializer<'de> {
        value: &'de [u8],
        marker: std::marker::PhantomData<&'de ()>,
    }
    
    impl<'de> BorrowedBytesDeserializer<'de> {
        pub fn new(value: &'de [u8]) -> Self {
            BorrowedBytesDeserializer {
                value,
                marker: std::marker::PhantomData,
            }
        }
    }

    let bytes: &[u8] = &[1, 2, 3, 4];
    let deserializer = BorrowedBytesDeserializer::new(bytes);
    assert_eq!(deserializer.value, bytes);
}

#[test]
fn test_new_borrowed_bytes_deserializer_empty() {
    struct BorrowedBytesDeserializer<'de> {
        value: &'de [u8],
        marker: std::marker::PhantomData<&'de ()>,
    }

    impl<'de> BorrowedBytesDeserializer<'de> {
        pub fn new(value: &'de [u8]) -> Self {
            BorrowedBytesDeserializer {
                value,
                marker: std::marker::PhantomData,
            }
        }
    }

    let bytes: &[u8] = &[];
    let deserializer = BorrowedBytesDeserializer::new(bytes);
    assert_eq!(deserializer.value, bytes);
}

#[test]
fn test_new_borrowed_bytes_deserializer_large() {
    struct BorrowedBytesDeserializer<'de> {
        value: &'de [u8],
        marker: std::marker::PhantomData<&'de ()>,
    }

    impl<'de> BorrowedBytesDeserializer<'de> {
        pub fn new(value: &'de [u8]) -> Self {
            BorrowedBytesDeserializer {
                value,
                marker: std::marker::PhantomData,
            }
        }
    }

    let bytes: &[u8] = &[5; 1024]; // Large array of bytes
    let deserializer = BorrowedBytesDeserializer::new(bytes);
    assert_eq!(deserializer.value.len(), 1024);
}

