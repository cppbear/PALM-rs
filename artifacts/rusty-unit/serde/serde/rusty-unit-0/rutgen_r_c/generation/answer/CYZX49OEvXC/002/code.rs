// Answer 0

#[test]
fn test_visit_bytes_content() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self
        where
            T: std::fmt::Display,
        {
            TestError
        }
    }

    let visitor = TagContentOtherFieldVisitor {
        tag: "tag_field",
        content: "content_field",
    };

    let result = visitor.visit_bytes("content_field".as_bytes());
    assert_eq!(result, Ok(TagContentOtherField::Content));
}

#[test]
fn test_visit_bytes_other() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self
        where
            T: std::fmt::Display,
        {
            TestError
        }
    }

    let visitor = TagContentOtherFieldVisitor {
        tag: "tag_field",
        content: "content_field",
    };

    let result = visitor.visit_bytes("other_field".as_bytes());
    assert_eq!(result, Ok(TagContentOtherField::Other));
}

