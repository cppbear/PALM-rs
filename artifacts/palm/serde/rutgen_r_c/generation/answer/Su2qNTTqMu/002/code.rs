// Answer 0

#[test]
fn test_serialize_some_ok() {
    struct SerializableValue;

    impl Serialize for SerializableValue {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str("test_string")
        }
    }

    struct TestError;

    impl ser::Error for TestError {}
    
    let serializer = ContentSerializer::<TestError> { error: PhantomData };
    let value = SerializableValue;

    let result = serializer.serialize_some(&value);
    assert!(result.is_ok());

    if let Ok(content) = result {
        match content {
            Content::Some(boxed_content) => match *boxed_content {
                Content::Str(s) => assert_eq!(s, "test_string"),
                _ => panic!("Expected a Content::Str variant."),
            },
            _ => panic!("Expected Content::Some variant."),
        }
    }
}

#[test]
fn test_serialize_some_non_serializable() {
    struct NonSerializableValue;

    struct TestError;

    impl ser::Error for TestError {}

    let serializer = ContentSerializer::<TestError> { error: PhantomData };
    let value = NonSerializableValue;

    let result = serializer.serialize_some(&value);
    assert!(result.is_err());
}

#[test]
fn test_serialize_some_none() {
    struct SerializableNone;

    impl Serialize for SerializableNone {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(TestError)
        }
    }

    struct TestError;

    impl ser::Error for TestError {}

    let serializer = ContentSerializer::<TestError> { error: PhantomData };
    let value = SerializableNone;

    let result = serializer.serialize_some(&value);
    assert!(result.is_err());
}

