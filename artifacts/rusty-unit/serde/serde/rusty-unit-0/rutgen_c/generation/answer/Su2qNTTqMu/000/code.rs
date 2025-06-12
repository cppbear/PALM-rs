// Answer 0

#[test]
fn test_serialize_some_with_valid_value() {
    struct TestValue;

    impl Serialize for TestValue {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str("test_value")
        }
    }

    let serializer = ContentSerializer::<()>;
    let result = serializer.serialize_some(&TestValue);
    assert!(result.is_ok());
    if let Ok(content) = result {
        if let Content::Some(boxed_content) = content {
            if let Content::Str(value) = *boxed_content {
                assert_eq!(value, "test_value");
            } else {
                panic!("Expected Content::Str");
            }
        } else {
            panic!("Expected Content::Some");
        }
    }
}

#[test]
#[should_panic(expected = "not supported")]
fn test_serialize_some_with_invalid_value() {
    struct InvalidValue;

    impl Serialize for InvalidValue {}

    let serializer = ContentSerializer::<()>;
    let _ = serializer.serialize_some(&InvalidValue);
}

