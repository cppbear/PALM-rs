// Answer 0

#[test]
fn test_end_with_remaining_elements() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self {
            TestError
        }
    }

    let iter = vec![1].into_iter().fuse(); // remaining = 1
    let count = 0;

    let seq_deserializer: SeqDeserializer<_, TestError> = SeqDeserializer {
        iter,
        count,
        marker: PhantomData,
    };

    let result = seq_deserializer.end();
}

#[test]
fn test_end_with_multiple_remaining_elements() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self {
            TestError
        }
    }

    let iter = vec![1, 2, 3].into_iter().fuse(); // remaining = 3
    let count = 0;

    let seq_deserializer: SeqDeserializer<_, TestError> = SeqDeserializer {
        iter,
        count,
        marker: PhantomData,
    };

    let result = seq_deserializer.end();
}

