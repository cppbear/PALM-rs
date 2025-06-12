// Answer 0

#[test]
fn test_serialize_i16() {
    struct TestError;
    
    impl ser::Error for TestError {
        fn custom<T: std::fmt::Display>(_msg: T) -> Self {
            TestError
        }
    }
    
    let serializer = ContentSerializer::<TestError> { error: std::marker::PhantomData };
    
    assert_eq!(serializer.serialize_i16(42).unwrap(), Content::I16(42));
}

#[test]
fn test_serialize_i16_negative() {
    struct TestError;
    
    impl ser::Error for TestError {
        fn custom<T: std::fmt::Display>(_msg: T) -> Self {
            TestError
        }
    }
    
    let serializer = ContentSerializer::<TestError> { error: std::marker::PhantomData };
    
    assert_eq!(serializer.serialize_i16(-1).unwrap(), Content::I16(-1));
}

