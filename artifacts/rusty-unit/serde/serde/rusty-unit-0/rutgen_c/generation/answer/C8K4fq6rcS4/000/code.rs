// Answer 0

#[test]
fn test_serialize_str_valid() {
    struct TestError;
    impl std::fmt::Debug for TestError {}
    impl ser::Error for TestError {}

    let serializer = ContentSerializer::<TestError> {
        error: PhantomData,
    };

    let result = serializer.serialize_str("test string").unwrap();
    assert_eq!(result, Content::String("test string".to_owned()));
}

#[test]
fn test_serialize_str_empty() {
    struct TestError;
    impl std::fmt::Debug for TestError {}
    impl ser::Error for TestError {}

    let serializer = ContentSerializer::<TestError> {
        error: PhantomData,
    };

    let result = serializer.serialize_str("").unwrap();
    assert_eq!(result, Content::String("".to_owned()));
}

