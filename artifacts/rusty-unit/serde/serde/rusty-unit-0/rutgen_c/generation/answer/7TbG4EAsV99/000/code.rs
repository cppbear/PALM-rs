// Answer 0

#[test]
fn test_serialize_field_bool() {
    struct DummyError;
    impl std::fmt::Debug for DummyError {}
    impl ser::Error for DummyError {}

    let mut serializer = SerializeStruct::<DummyError> {
        name: "test",
        fields: Vec::new(),
        error: PhantomData,
    };

    let value = true;
    let result = serializer.serialize_field("is_true", &value);
    
    assert!(result.is_ok());
    assert_eq!(serializer.fields.len(), 1);
    assert_eq!(serializer.fields[0].0, "is_true");
}

#[test]
fn test_serialize_field_u8() {
    struct DummyError;
    impl std::fmt::Debug for DummyError {}
    impl ser::Error for DummyError {}

    let mut serializer = SerializeStruct::<DummyError> {
        name: "test",
        fields: Vec::new(),
        error: PhantomData,
    };

    let value = 42u8;
    let result = serializer.serialize_field("value", &value);
    
    assert!(result.is_ok());
    assert_eq!(serializer.fields.len(), 1);
    assert_eq!(serializer.fields[0].0, "value");
}

#[test]
fn test_serialize_field_string() {
    struct DummyError;
    impl std::fmt::Debug for DummyError {}
    impl ser::Error for DummyError {}

    let mut serializer = SerializeStruct::<DummyError> {
        name: "test",
        fields: Vec::new(),
        error: PhantomData,
    };

    let value = "Hello, World!";
    let result = serializer.serialize_field("greeting", &value);
    
    assert!(result.is_ok());
    assert_eq!(serializer.fields.len(), 1);
    assert_eq!(serializer.fields[0].0, "greeting");
}

#[test]
fn test_serialize_field_none() {
    struct DummyError;
    impl std::fmt::Debug for DummyError {}
    impl ser::Error for DummyError {}

    let mut serializer = SerializeStruct::<DummyError> {
        name: "test",
        fields: Vec::new(),
        error: PhantomData,
    };

    let value: Option<&str> = None;
    let result = serializer.serialize_field("empty", &value);
    
    assert!(result.is_ok());
    assert_eq!(serializer.fields.len(), 1);
    assert_eq!(serializer.fields[0].0, "empty");
} 

#[should_panic]
#[test]
fn test_serialize_field_invalid() {
    struct DummyError;
    impl std::fmt::Debug for DummyError {}
    impl ser::Error for DummyError {}

    struct InvalidType;

    let mut serializer = SerializeStruct::<DummyError> {
        name: "test",
        fields: Vec::new(),
        error: PhantomData,
    };

    let result = serializer.serialize_field("invalid", &InvalidType);
    assert!(result.is_err());
}

