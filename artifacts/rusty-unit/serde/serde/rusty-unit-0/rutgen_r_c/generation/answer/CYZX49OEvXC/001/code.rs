// Answer 0

#[test]
fn test_visit_bytes_tag() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(_: T) -> Self {
            TestError
        }
    }

    let visitor = TagContentOtherFieldVisitor {
        tag: "tag_field",
        content: "content_field",
    };

    let result = visitor.visit_bytes(b"tag_field");
    assert_eq!(result, Ok(TagContentOtherField::Tag));
}

#[test]
fn test_visit_bytes_content() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(_: T) -> Self {
            TestError
        }
    }

    let visitor = TagContentOtherFieldVisitor {
        tag: "tag_field",
        content: "content_field",
    };

    let result = visitor.visit_bytes(b"content_field");
    assert_eq!(result, Ok(TagContentOtherField::Content));
}

#[test]
fn test_visit_bytes_other() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(_: T) -> Self {
            TestError
        }
    }

    let visitor = TagContentOtherFieldVisitor {
        tag: "tag_field",
        content: "content_field",
    };

    let result = visitor.visit_bytes(b"other_field");
    assert_eq!(result, Ok(TagContentOtherField::Other));
}

