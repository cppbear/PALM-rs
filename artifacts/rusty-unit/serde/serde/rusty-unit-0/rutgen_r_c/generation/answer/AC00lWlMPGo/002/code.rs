// Answer 0

#[test]
fn test_serialize_value_success() {
    struct MockError;
    impl ser::Error for MockError {
        fn custom<T>(_: T) -> Self {
            MockError
        }
    }

    let mut serializer = SerializeMap::<MockError> {
        entries: Vec::new(),
        key: Some(Content::String("key".to_string())),
        error: PhantomData,
    };

    let value = Content::U32(42);
    let result = serializer.serialize_value(&value);
    
    assert!(result.is_ok());
    assert_eq!(serializer.entries.len(), 1);
    assert_eq!(serializer.entries[0], (Content::String("key".to_string()), value));
}

#[test]
#[should_panic(expected = "serialize_value called before serialize_key")]
fn test_serialize_value_no_key() {
    struct MockError;
    impl ser::Error for MockError {
        fn custom<T>(_: T) -> Self {
            MockError
        }
    }

    let mut serializer = SerializeMap::<MockError> {
        entries: Vec::new(),
        key: None,
        error: PhantomData,
    };

    let value = Content::U32(42);
    let _ = serializer.serialize_value(&value);
}

