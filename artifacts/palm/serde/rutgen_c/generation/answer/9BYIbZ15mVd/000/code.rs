// Answer 0

#[test]
fn test_serialize_field_bool() {
    struct MockError;
    impl std::fmt::Debug for MockError {}
    impl serde::ser::Error for MockError {
        fn custom<T>(_msg: T) -> Self {
            MockError
        }
    }
    
    struct DummyStruct {
        value: bool,
    }
    
    impl serde::Serialize for DummyStruct {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_bool(self.value)
        }
    }
    
    let mut serializer = SerializeTupleVariant::<MockError> {
        name: "test",
        variant_index: 0,
        variant: "variant",
        fields: Vec::new(),
        error: PhantomData,
    };
    
    let result = serializer.serialize_field(&DummyStruct { value: true });
    assert!(result.is_ok());
    assert_eq!(serializer.fields.len(), 1);
    if let Some(Content::Bool(value)) = serializer.fields.get(0) {
        assert!(*value);
    } else {
        panic!("Expected first field to be Bool");
    }
}

#[test]
fn test_serialize_field_u8() {
    struct MockError;
    impl std::fmt::Debug for MockError {}
    impl serde::ser::Error for MockError {
        fn custom<T>(_msg: T) -> Self {
            MockError
        }
    }
    
    struct DummyStruct {
        value: u8,
    }
    
    impl serde::Serialize for DummyStruct {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_u8(self.value)
        }
    }
    
    let mut serializer = SerializeTupleVariant::<MockError> {
        name: "test",
        variant_index: 1,
        variant: "variant",
        fields: Vec::new(),
        error: PhantomData,
    };
    
    let result = serializer.serialize_field(&DummyStruct { value: 42 });
    assert!(result.is_ok());
    assert_eq!(serializer.fields.len(), 1);
    if let Some(Content::U8(value)) = serializer.fields.get(0) {
        assert_eq!(*value, 42);
    } else {
        panic!("Expected first field to be U8");
    }
}

#[test]
fn test_serialize_field_string() {
    struct MockError;
    impl std::fmt::Debug for MockError {}
    impl serde::ser::Error for MockError {
        fn custom<T>(_msg: T) -> Self {
            MockError
        }
    }
    
    struct DummyStruct {
        value: String,
    }
    
    impl serde::Serialize for DummyStruct {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&self.value)
        }
    }
    
    let mut serializer = SerializeTupleVariant::<MockError> {
        name: "test",
        variant_index: 2,
        variant: "variant",
        fields: Vec::new(),
        error: PhantomData,
    };
    
    let result = serializer.serialize_field(&DummyStruct { value: "Hello".to_string() });
    assert!(result.is_ok());
    assert_eq!(serializer.fields.len(), 1);
    if let Some(Content::String(value)) = serializer.fields.get(0) {
        assert_eq!(value, "Hello");
    } else {
        panic!("Expected first field to be String");
    }
}

