// Answer 0

#[test]
fn test_serialize_char_valid() {
    struct TestError;
    impl std::fmt::Debug for TestError {}
    impl ser::Error for TestError {}

    let serializer = ContentSerializer::<TestError> { error: PhantomData };
    
    let char_to_serialize = 'A';
    let result = serializer.serialize_char(char_to_serialize);
    
    assert_eq!(result, Ok(Content::Char(char_to_serialize)));
}

#[test]
fn test_serialize_char_empty() {
    struct TestError;
    impl std::fmt::Debug for TestError {}
    impl ser::Error for TestError {}

    let serializer = ContentSerializer::<TestError> { error: PhantomData };
    
    let char_to_serialize = '\0'; // Adding a null character
    let result = serializer.serialize_char(char_to_serialize);
    
    assert_eq!(result, Ok(Content::Char(char_to_serialize)));
}

