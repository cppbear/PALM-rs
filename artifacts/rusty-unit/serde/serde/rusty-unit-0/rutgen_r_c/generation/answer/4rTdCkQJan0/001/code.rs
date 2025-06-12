// Answer 0

#[test]
fn test_visit_str_tag() {
    struct TestError;

    impl de::Error for TestError {
        fn invalid_value(_: Unexpected, _: &dyn Visitor) -> Self {
            TestError
        }
    }

    let visitor = TagOrContentFieldVisitor { 
        tag: "my_tag", 
        content: "my_content" 
    };

    let result: Result<TagOrContentField, TestError> = visitor.visit_str("my_tag");
    assert_eq!(result, Ok(TagOrContentField::Tag));
}

#[test]
fn test_visit_str_content() {
    struct TestError;

    impl de::Error for TestError {
        fn invalid_value(_: Unexpected, _: &dyn Visitor) -> Self {
            TestError
        }
    }

    let visitor = TagOrContentFieldVisitor { 
        tag: "my_tag", 
        content: "my_content" 
    };

    let result: Result<TagOrContentField, TestError> = visitor.visit_str("my_content");
    assert_eq!(result, Ok(TagOrContentField::Content));
}

#[test]
fn test_visit_str_invalid() {
    struct TestError;

    impl de::Error for TestError {
        fn invalid_value(_: Unexpected, _: &dyn Visitor) -> Self {
            TestError
        }
    }

    let visitor = TagOrContentFieldVisitor { 
        tag: "my_tag", 
        content: "my_content" 
    };

    let result: Result<TagOrContentField, TestError> = visitor.visit_str("invalid_field");
    assert!(result.is_err());
}

