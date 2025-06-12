// Answer 0

#[test]
fn test_serialize_i64() {
    struct TestError;
    impl ser::Error for TestError {}

    let serializer = ContentSerializer::<TestError> {
        error: PhantomData,
    };

    let result = serializer.serialize_i64(42);
    assert_eq!(result, Ok(Content::I64(42)));
}

#[test]
fn test_serialize_i64_negative() {
    struct TestError;
    impl ser::Error for TestError {}

    let serializer = ContentSerializer::<TestError> {
        error: PhantomData,
    };

    let result = serializer.serialize_i64(-1);
    assert_eq!(result, Ok(Content::I64(-1)));
}

#[test]
fn test_serialize_i64_zero() {
    struct TestError;
    impl ser::Error for TestError {}

    let serializer = ContentSerializer::<TestError> {
        error: PhantomData,
    };

    let result = serializer.serialize_i64(0);
    assert_eq!(result, Ok(Content::I64(0)));
}

