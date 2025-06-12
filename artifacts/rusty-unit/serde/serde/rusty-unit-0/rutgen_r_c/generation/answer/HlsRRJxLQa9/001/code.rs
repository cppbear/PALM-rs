// Answer 0

#[test]
fn test_serialize_i32_positive_value() {
    struct TestError;

    impl ser::Error for TestError {}

    let serializer = ContentSerializer::<TestError> { error: PhantomData };
    let result = serializer.serialize_i32(42);
    assert_eq!(result, Ok(Content::I32(42)));
}

#[test]
fn test_serialize_i32_negative_value() {
    struct TestError;

    impl ser::Error for TestError {}

    let serializer = ContentSerializer::<TestError> { error: PhantomData };
    let result = serializer.serialize_i32(-42);
    assert_eq!(result, Ok(Content::I32(-42)));
}

#[test]
fn test_serialize_i32_zero() {
    struct TestError;

    impl ser::Error for TestError {}

    let serializer = ContentSerializer::<TestError> { error: PhantomData };
    let result = serializer.serialize_i32(0);
    assert_eq!(result, Ok(Content::I32(0)));
}

