// Answer 0

#[test]
fn test_visit_bool() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
        // Other required methods would need to be stubbed, omitted for brevity
    }

    let visitor = TagOrContentVisitor {
        name: "test_tag",
        value: PhantomData,
    };

    let result = visitor.visit_bool::<TestError>(true);
    match result {
        Ok(TagOrContent::Content(Content::Bool(value))) => {
            assert!(value);
        }
        _ => panic!("Expected Content::Bool with value true"),
    }
}

#[test]
fn test_visit_bool_invalid() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
        // Other required methods would need to be stubbed, omitted for brevity
    }

    let visitor = TagOrContentVisitor {
        name: "test_tag",
        value: PhantomData,
    };

    let result = visitor.visit_bool::<TestError>(true);
    assert!(result.is_ok());

    let invalid_result = visitor.visit_str::<TestError>("not_a_bool");
    match invalid_result {
        Err(_) => {} // Expected an error
        _ => panic!("Expected an error due to invalid type"),
    }
}

