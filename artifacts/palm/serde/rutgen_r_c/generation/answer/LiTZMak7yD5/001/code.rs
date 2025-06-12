// Answer 0

#[test]
fn test_serialize_bool_true() {
    struct TestError;
    
    impl serde::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    let serializer = ContentSerializer::<TestError> {
        error: std::marker::PhantomData,
    };
    
    let result = serializer.serialize_bool(true);
    assert_eq!(result, Ok(Content::Bool(true)));
}

#[test]
fn test_serialize_bool_false() {
    struct TestError;
    
    impl serde::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    let serializer = ContentSerializer::<TestError> {
        error: std::marker::PhantomData,
    };
    
    let result = serializer.serialize_bool(false);
    assert_eq!(result, Ok(Content::Bool(false)));
}

