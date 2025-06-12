// Answer 0

#[test]
fn test_borrowed_bytes_deserializer_new() {
    use std::marker::PhantomData;

    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display { TestError }
    }

    let data: &[u8] = b"test data";
    let deserializer: BorrowedBytesDeserializer<TestError> = BorrowedBytesDeserializer::new(data);

    assert_eq!(deserializer.value, b"test data");
}

#[test]
fn test_borrowed_bytes_deserializer_new_empty() {
    use std::marker::PhantomData;

    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display { TestError }
    }

    let data: &[u8] = &[];
    let deserializer: BorrowedBytesDeserializer<TestError> = BorrowedBytesDeserializer::new(data);

    assert!(deserializer.value.is_empty());
}

#[test]
fn test_borrowed_bytes_deserializer_new_large_data() {
    use std::marker::PhantomData;

    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display { TestError }
    }

    let data: &[u8] = &[0u8; 1024]; // large array
    let deserializer: BorrowedBytesDeserializer<TestError> = BorrowedBytesDeserializer::new(data);

    assert_eq!(deserializer.value.len(), 1024);
}

