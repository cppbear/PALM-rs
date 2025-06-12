// Answer 0

#[test]
fn test_visit_i32() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display { TestError }
    }
    
    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, TestError> = visitor.visit_i32(42);
    
    match result {
        Ok(content) => match content {
            Content::I32(val) => assert_eq!(val, 42),
            _ => panic!("Expected I32 variant."),
        },
        Err(_) => panic!("Expected success but got an error."),
    }
}

#[test]
fn test_visit_i32_negative() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display { TestError }
    }
    
    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, TestError> = visitor.visit_i32(-30);
    
    match result {
        Ok(content) => match content {
            Content::I32(val) => assert_eq!(val, -30),
            _ => panic!("Expected I32 variant."),
        },
        Err(_) => panic!("Expected success but got an error."),
    }
}

#[test]
fn test_visit_i32_zero() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display { TestError }
    }
    
    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, TestError> = visitor.visit_i32(0);
    
    match result {
        Ok(content) => match content {
            Content::I32(val) => assert_eq!(val, 0),
            _ => panic!("Expected I32 variant."),
        },
        Err(_) => panic!("Expected success but got an error."),
    }
}

