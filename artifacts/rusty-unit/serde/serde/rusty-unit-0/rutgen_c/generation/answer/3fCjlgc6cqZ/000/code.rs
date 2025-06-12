// Answer 0

#[test]
fn test_serialize_u16() {
    struct TestError;
    impl ser::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
            TestError
        }
    }

    let serializer = ContentSerializer::<TestError> {
        error: PhantomData,
    };

    let result = serializer.serialize_u16(42);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Content::U16(42));
}

#[test]
fn test_serialize_u16_zero() {
    struct TestError;
    impl ser::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
            TestError
        }
    }

    let serializer = ContentSerializer::<TestError> {
        error: PhantomData,
    };

    let result = serializer.serialize_u16(0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Content::U16(0));
}

#[test]
fn test_serialize_u16_boundary() {
    struct TestError;
    impl ser::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
            TestError
        }
    }

    let serializer = ContentSerializer::<TestError> {
        error: PhantomData,
    };

    let result = serializer.serialize_u16(u16::MAX);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Content::U16(u16::MAX));
}

