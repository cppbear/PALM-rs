// Answer 0

#[test]
fn test_serialize_element_bool() {
    struct TestError;
    impl ser::Error for TestError {}
    
    let mut serialize_tuple: SerializeTuple<TestError> = SerializeTuple {
        elements: Vec::new(),
        error: PhantomData,
    };
    
    let result = serialize_tuple.serialize_element(&true);
    assert!(result.is_ok());
    assert_eq!(serialize_tuple.elements.len(), 1);
    match &serialize_tuple.elements[0] {
        Content::Bool(value) => assert!(*value),
        _ => panic!("Expected a Bool variant"),
    }
}

#[test]
fn test_serialize_element_u8() {
    struct TestError;
    impl ser::Error for TestError {}
    
    let mut serialize_tuple: SerializeTuple<TestError> = SerializeTuple {
        elements: Vec::new(),
        error: PhantomData,
    };
    
    let result = serialize_tuple.serialize_element(&255u8);
    assert!(result.is_ok());
    assert_eq!(serialize_tuple.elements.len(), 1);
    match &serialize_tuple.elements[0] {
        Content::U8(value) => assert_eq!(*value, 255),
        _ => panic!("Expected a U8 variant"),
    }
}

#[test]
fn test_serialize_element_string() {
    struct TestError;
    impl ser::Error for TestError {}
    
    let mut serialize_tuple: SerializeTuple<TestError> = SerializeTuple {
        elements: Vec::new(),
        error: PhantomData,
    };
    
    let result = serialize_tuple.serialize_element(&"Hello".to_string());
    assert!(result.is_ok());
    assert_eq!(serialize_tuple.elements.len(), 1);
    match &serialize_tuple.elements[0] {
        Content::String(value) => assert_eq!(value, "Hello"),
        _ => panic!("Expected a String variant"),
    }
}

#[test]
fn test_serialize_empty_tuple() {
    struct TestError;
    impl ser::Error for TestError {}
    
    let mut serialize_tuple: SerializeTuple<TestError> = SerializeTuple {
        elements: Vec::new(),
        error: PhantomData,
    };
    
    let result = serialize_tuple.end();
    assert!(result.is_ok());
    let content = result.unwrap();
    assert!(matches!(content, Content::Seq(elements) if elements.is_empty()));
}

