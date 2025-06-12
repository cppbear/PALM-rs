// Answer 0

#[test]
fn test_serialize_bytes_empty() {
    struct TestError;
    impl Error for TestError {}

    let serializer = ContentSerializer::<TestError> { error: PhantomData };
    let result = serializer.serialize_bytes(&[]);
    assert_eq!(result, Ok(Content::Bytes(Vec::new())));
}

#[test]
fn test_serialize_bytes_non_empty() {
    struct TestError;
    impl Error for TestError {}

    let serializer = ContentSerializer::<TestError> { error: PhantomData };
    let bytes = vec![1, 2, 3, 4];
    let result = serializer.serialize_bytes(&bytes);
    assert_eq!(result, Ok(Content::Bytes(bytes.clone())));
}

#[test]
fn test_serialize_bytes_single_element() {
    struct TestError;
    impl Error for TestError {}

    let serializer = ContentSerializer::<TestError> { error: PhantomData };
    let bytes = vec![42];
    let result = serializer.serialize_bytes(&bytes);
    assert_eq!(result, Ok(Content::Bytes(bytes.clone())));
}

