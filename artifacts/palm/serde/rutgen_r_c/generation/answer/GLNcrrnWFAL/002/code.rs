// Answer 0

#[test]
fn test_visit_byte_buf_not_equal() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T: std::fmt::Display>(msg: T) -> Self {
            TestError
        }
    }

    let visitor = TagOrContentVisitor {
        name: "expected_name",
        value: PhantomData,
    };

    let input_value: Vec<u8> = b"unexpected_value".to_vec();
    
    let result = visitor.visit_byte_buf(input_value);

    match result {
        Ok(TagOrContent::Content(Content::ByteBuf(_))) => (),
        _ => panic!("Expected a Content variant, received {:?}", result),
    }
}

#[test]
fn test_visit_byte_buf_equal() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T: std::fmt::Display>(msg: T) -> Self {
            TestError
        }
    }

    let visitor = TagOrContentVisitor {
        name: "test_name",
        value: PhantomData,
    };

    let input_value: Vec<u8> = b"test_name".to_vec();

    let result = visitor.visit_byte_buf(input_value);

    match result {
        Ok(TagOrContent::Tag) => (),
        _ => panic!("Expected a Tag variant, received {:?}", result),
    }
}

