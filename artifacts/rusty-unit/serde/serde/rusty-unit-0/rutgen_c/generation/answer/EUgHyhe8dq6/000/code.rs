// Answer 0

#[test]
fn test_visit_borrowed_bytes() {
    struct DummyError;
    impl de::Error for DummyError {
        fn custom<T: std::fmt::Display>(msg: T) -> Self {
            DummyError
        }
    }
    
    let visitor = ContentVisitor { value: PhantomData };
    let input_bytes: &[u8] = b"test";
    let result = visitor.visit_borrowed_bytes(input_bytes);
    
    match result {
        Ok(content) => match content {
            Content::Bytes(bytes) => {
                assert_eq!(bytes, input_bytes);
            }
            _ => panic!("Expected Content::Bytes"),
        },
        Err(_) => panic!("Expected Ok but found Err"),
    }
}

#[test]
fn test_visit_borrowed_bytes_empty() {
    struct DummyError;
    impl de::Error for DummyError {
        fn custom<T: std::fmt::Display>(msg: T) -> Self {
            DummyError
        }
    }
    
    let visitor = ContentVisitor { value: PhantomData };
    let input_bytes: &[u8] = b"";
    let result = visitor.visit_borrowed_bytes(input_bytes);
    
    match result {
        Ok(content) => match content {
            Content::Bytes(bytes) => {
                assert_eq!(bytes, input_bytes);
            }
            _ => panic!("Expected Content::Bytes"),
        },
        Err(_) => panic!("Expected Ok but found Err"),
    }
}

