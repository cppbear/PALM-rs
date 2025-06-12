// Answer 0

#[test]
fn test_visit_string_with_tag() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display { TestError }
    }

    let visitor = TagOrContentVisitor {
        name: "test_tag",
        value: std::marker::PhantomData,
    };

    let result = visitor.visit_string("test_tag".to_string()).unwrap();
    match result {
        TagOrContent::Tag => {},
        _ => panic!("Expected Tag variant"),
    }
}

#[test]
fn test_visit_string_with_content() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display { TestError }
    }

    let visitor = TagOrContentVisitor {
        name: "test_tag",
        value: std::marker::PhantomData,
    };

    let result = visitor.visit_string("not_a_tag".to_string()).unwrap();
    if let TagOrContent::Content(content) = result {
        if let Content::String(val) = content {
            assert_eq!(val, "not_a_tag");
        } else {
            panic!("Expected Content variant with String");
        }
    } else {
        panic!("Expected Content variant");
    }
}

