// Answer 0

#[test]
fn test_bytes_deserializer_into_deserializer() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T: std::fmt::Display>(msg: T) -> Self {
            TestError
        }
    }

    let value: &[u8] = b"test";
    let deserializer = BytesDeserializer {
        value,
        marker: std::marker::PhantomData,
    };

    let result = deserializer.into_deserializer();
    assert_eq!(result.value, value);
}

#[test]
fn test_bool_deserializer_into_deserializer() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T: std::fmt::Display>(msg: T) -> Self {
            TestError
        }
    }

    let deserializer = BoolDeserializer::new(true);
    let result = deserializer.into_deserializer();
    assert_eq!(result.value, true);
}

