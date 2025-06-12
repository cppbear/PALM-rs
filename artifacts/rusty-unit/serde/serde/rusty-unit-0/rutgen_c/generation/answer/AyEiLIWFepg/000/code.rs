// Answer 0

#[test]
fn test_serialize_f32() {
    struct MockError;
    impl ser::Error for MockError {}

    let serializer = ContentSerializer::<MockError> { error: PhantomData };
    let result = serializer.serialize_f32(3.14);
    
    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            Content::F32(value) => assert_eq!(value, 3.14),
            _ => panic!("Expected Content::F32"),
        }
    }
}

#[test]
fn test_serialize_f32_negative() {
    struct MockError;
    impl ser::Error for MockError {}

    let serializer = ContentSerializer::<MockError> { error: PhantomData };
    let result = serializer.serialize_f32(-2.71);
    
    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            Content::F32(value) => assert_eq!(value, -2.71),
            _ => panic!("Expected Content::F32"),
        }
    }
}

#[test]
fn test_serialize_f32_zero() {
    struct MockError;
    impl ser::Error for MockError {}

    let serializer = ContentSerializer::<MockError> { error: PhantomData };
    let result = serializer.serialize_f32(0.0);
    
    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            Content::F32(value) => assert_eq!(value, 0.0),
            _ => panic!("Expected Content::F32"),
        }
    }
}

