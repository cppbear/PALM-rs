// Answer 0

#[test]
fn test_visit_bytes_tag() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    let visitor = TagOrContentFieldVisitor {
        tag: "tag_field",
        content: "content_field",
    };

    let result = visitor.visit_bytes(b"tag_field").map_err(|_| TestError);

    assert_eq!(result, Ok(TagOrContentField::Tag));
}

#[test]
fn test_visit_bytes_content() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    let visitor = TagOrContentFieldVisitor {
        tag: "tag_field",
        content: "content_field",
    };

    let result = visitor.visit_bytes(b"content_field").map_err(|_| TestError);

    assert_eq!(result, Ok(TagOrContentField::Content));
}

#[test]
fn test_visit_bytes_invalid() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    let visitor = TagOrContentFieldVisitor {
        tag: "tag_field",
        content: "content_field",
    };

    let result = visitor.visit_bytes(b"invalid_field").map_err(|_| TestError);

    assert!(result.is_err());
}

