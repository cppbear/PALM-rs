// Answer 0

#[test]
fn test_visit_borrowed_str_equal_name() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self { TestError }
    }

    let visitor = TagOrContentVisitor {
        name: "test_name",
        value: PhantomData,
    };

    let result = visitor.visit_borrowed_str("test_name");

    assert!(result.is_ok());
    if let Ok(tag_or_content) = result {
        match tag_or_content {
            TagOrContent::Tag => (),
            _ => panic!("Expected TagOrContent::Tag"),
        }
    } else {
        panic!("Expected Ok result");
    }
}

