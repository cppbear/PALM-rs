// Answer 0

#[test]
fn test_is_with_matching_type() {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    struct TestError;
    
    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestError occurred")
        }
    }
    
    impl Error for TestError {}

    let my_error: Box<dyn Error> = Box::new(TestError);
    let result = my_error.is::<TestError>();
    assert!(result);
}

#[test]
fn test_is_with_non_matching_type() {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    struct TestError;
    
    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestError occurred")
        }
    }
    
    impl Error for TestError {}

    #[derive(Debug)]
    struct AnotherError;
    
    impl fmt::Display for AnotherError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "AnotherError occurred")
        }
    }
    
    impl Error for AnotherError {}

    let my_error: Box<dyn Error> = Box::new(TestError);
    let result = my_error.is::<AnotherError>();
    assert!(!result);
}

#[test]
#[should_panic]
fn test_is_with_invalid_type() {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    struct TestError;
    
    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestError occurred")
        }
    }
    
    impl Error for TestError {}

    let my_error: Box<dyn Error> = Box::new(TestError);
    // Here we are trying to access an invalid type which should cause a panic.
    let _ = my_error.is::<i32>();
}

