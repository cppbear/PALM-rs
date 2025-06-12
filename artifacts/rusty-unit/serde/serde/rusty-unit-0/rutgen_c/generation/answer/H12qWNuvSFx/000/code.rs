// Answer 0

#[test]
fn test_serialize_f64() {
    struct TestError;
    impl ser::Error for TestError {}

    let serializer: ContentSerializer<TestError> = ContentSerializer {
        error: PhantomData,
    };

    let result = serializer.serialize_f64(3.14);
    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            Content::F64(value) => assert_eq!(value, 3.14),
            _ => panic!("Expected Content::F64"),
        }
    }
}

#[test]
fn test_serialize_f64_negative() {
    struct TestError;
    impl ser::Error for TestError {}

    let serializer: ContentSerializer<TestError> = ContentSerializer {
        error: PhantomData,
    };

    let result = serializer.serialize_f64(-1.0);
    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            Content::F64(value) => assert_eq!(value, -1.0),
            _ => panic!("Expected Content::F64"),
        }
    }
}

#[test]
fn test_serialize_f64_zero() {
    struct TestError;
    impl ser::Error for TestError {}

    let serializer: ContentSerializer<TestError> = ContentSerializer {
        error: PhantomData,
    };

    let result = serializer.serialize_f64(0.0);
    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            Content::F64(value) => assert_eq!(value, 0.0),
            _ => panic!("Expected Content::F64"),
        }
    }
}

