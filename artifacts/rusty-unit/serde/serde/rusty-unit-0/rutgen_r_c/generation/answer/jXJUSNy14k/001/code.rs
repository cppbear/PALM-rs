// Answer 0

#[test]
fn test_serialize_tuple_struct_with_zero_length() {
    struct DummyError;
    impl ser::Error for DummyError {}

    let serializer = ContentSerializer::<DummyError> { error: PhantomData };

    let result = serializer.serialize_tuple_struct("TestStruct", 0);
    assert!(result.is_ok());

    let tuple_struct = result.unwrap();
    assert_eq!(tuple_struct.name, "TestStruct");
    assert_eq!(tuple_struct.fields.capacity(), 0);
}

#[test]
fn test_serialize_tuple_struct_with_non_zero_length() {
    struct DummyError;
    impl ser::Error for DummyError {}

    let serializer = ContentSerializer::<DummyError> { error: PhantomData };

    let result = serializer.serialize_tuple_struct("TestStruct", 5);
    assert!(result.is_ok());

    let tuple_struct = result.unwrap();
    assert_eq!(tuple_struct.name, "TestStruct");
    assert_eq!(tuple_struct.fields.capacity(), 5);
}

#[test]
fn test_serialize_tuple_struct_with_large_length() {
    struct DummyError;
    impl ser::Error for DummyError {}

    let serializer = ContentSerializer::<DummyError> { error: PhantomData };

    let result = serializer.serialize_tuple_struct("LargeStruct", usize::MAX);
    assert!(result.is_ok());

    let tuple_struct = result.unwrap();
    assert_eq!(tuple_struct.name, "LargeStruct");
    assert!(tuple_struct.fields.capacity() > 0); // Check that it doesn't panic and has some capacity
}

