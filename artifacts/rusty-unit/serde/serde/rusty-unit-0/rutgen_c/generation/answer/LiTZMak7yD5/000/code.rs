// Answer 0

#[test]
fn test_serialize_bool() {
    struct TestError;
    impl std::fmt::Debug for TestError {}
    impl serde::ser::Error for TestError {
        fn custom<T>(_: T) -> Self {
            TestError
        }
    }

    let serializer = ContentSerializer::<TestError> { error: PhantomData };

    let result = serializer.serialize_bool(true);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Content::Bool(true));

    let result = serializer.serialize_bool(false);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Content::Bool(false));
}

