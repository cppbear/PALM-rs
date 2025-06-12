// Answer 0

#[test]
fn test_serialize_struct_empty() {
    struct TestError;
    impl std::fmt::Debug for TestError {}
    impl serde::ser::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    let serializer = ContentSerializer::<TestError> { error: std::marker::PhantomData };
    let result = serializer.serialize_struct("TestStruct", 0);
    assert!(result.is_ok());
    let serialize_struct = result.unwrap();
    assert_eq!(serialize_struct.name, "TestStruct");
    assert_eq!(serialize_struct.fields.len(), 0);
}

#[test]
fn test_serialize_struct_non_empty() {
    struct TestError;
    impl std::fmt::Debug for TestError {}
    impl serde::ser::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    let serializer = ContentSerializer::<TestError> { error: std::marker::PhantomData };
    let result = serializer.serialize_struct("TestStruct", 5);
    assert!(result.is_ok());
    let serialize_struct = result.unwrap();
    assert_eq!(serialize_struct.name, "TestStruct");
    assert_eq!(serialize_struct.fields.capacity(), 5);
}

