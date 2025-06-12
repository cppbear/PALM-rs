// Answer 0

#[test]
fn test_seq_deserializer_end_with_non_zero_remaining() {
    use std::iter::once;

    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    let deserializer: SeqDeserializer<_, TestError> = SeqDeserializer {
        iter: once(1).fuse(), // Simulates one extra element remaining
        count: 0,
        marker: PhantomData,
    };

    let result = deserializer.end();
    assert!(result.is_err());
}

#[test]
fn test_seq_deserializer_end_with_zero_remaining() {
    use std::iter::empty;

    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    let deserializer: SeqDeserializer<_, TestError> = SeqDeserializer {
        iter: empty().fuse(), // Simulates no remaining elements
        count: 0,
        marker: PhantomData,
    };

    let result = deserializer.end();
    assert!(result.is_ok());
}

