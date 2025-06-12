// Answer 0

#[test]
fn test_visit_borrowed_str() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
            TestError
        }
    }

    let visitor = ContentVisitor { value: PhantomData::<Content> };
    let input = "test string";

    let result: Result<Content, TestError> = visitor.visit_borrowed_str(input);
    
    match result {
        Ok(Content::Str(s)) => assert_eq!(s, input),
        _ => panic!("Expected Content::Str but got a different result"),
    }
}

#[test]
fn test_visit_borrowed_str_empty() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
            TestError
        }
    }

    let visitor = ContentVisitor { value: PhantomData::<Content> };
    let input = "";

    let result: Result<Content, TestError> = visitor.visit_borrowed_str(input);
    
    match result {
        Ok(Content::Str(s)) => assert_eq!(s, input),
        _ => panic!("Expected Content::Str but got a different result"),
    }
}

