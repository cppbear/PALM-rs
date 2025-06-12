// Answer 0

#[test]
fn test_serialize_element_bool() {
    struct MockError;
    impl std::fmt::Debug for MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "MockError")
        }
    }
    impl ser::Error for MockError {}

    struct SimpleBool(bool);

    impl ser::Serialize for SimpleBool {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.serialize_bool(self.0)
        }
    }

    let mut serializer = SerializeSeq::<MockError> { elements: Vec::new(), error: PhantomData };
    let value = SimpleBool(true);
    let result = serializer.serialize_element(&value);

    assert!(result.is_ok());
    assert_eq!(serializer.elements.len(), 1);
    if let Content::Bool(v) = &serializer.elements[0] {
        assert!(*v);
    } else {
        panic!("Expected a Bool content");
    }
}

#[test]
fn test_serialize_element_u8() {
    struct MockError;
    impl std::fmt::Debug for MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "MockError")
        }
    }
    impl ser::Error for MockError {}

    struct SimpleU8(u8);

    impl ser::Serialize for SimpleU8 {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.serialize_u8(self.0)
        }
    }

    let mut serializer = SerializeSeq::<MockError> { elements: Vec::new(), error: PhantomData };
    let value = SimpleU8(42);
    let result = serializer.serialize_element(&value);

    assert!(result.is_ok());
    assert_eq!(serializer.elements.len(), 1);
    if let Content::U8(v) = &serializer.elements[0] {
        assert_eq!(*v, 42);
    } else {
        panic!("Expected a U8 content");
    }
}

#[test]
#[should_panic(expected = "Expected a U8 content")]
fn test_serialize_element_invalid_type() {
    struct MockError;
    impl std::fmt::Debug for MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "MockError")
        }
    }
    impl ser::Error for MockError {}

    struct InvalidType;

    impl ser::Serialize for InvalidType {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            Err(MockError)
        }
    }

    let mut serializer = SerializeSeq::<MockError> { elements: Vec::new(), error: PhantomData };
    let value = InvalidType;
    let result = serializer.serialize_element(&value);
    assert!(result.is_err()); // This should be an error
}

