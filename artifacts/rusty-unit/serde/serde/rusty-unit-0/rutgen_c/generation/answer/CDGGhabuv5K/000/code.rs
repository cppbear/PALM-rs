// Answer 0

#[test]
fn test_serialize_unit() {
    struct TestError;
    impl ser::Error for TestError {}

    let serializer = ContentSerializer {
        error: PhantomData::<TestError>,
    };

    let result: Result<Content, TestError> = serializer.serialize_unit();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Content::Unit);
}

