// Answer 0

#[test]
fn test_visit_bytes_matches_name() {
    struct TestError;

    impl de::Error for TestError {}

    let visitor = TagOrContentVisitor {
        name: "test",
        value: PhantomData,
    };
    
    let result = visitor.visit_bytes(b"test");
    
    match result {
        Ok(TagOrContent::Tag) => (),
        _ => panic!("Expected TagOrContent::Tag"),
    }
}

#[test]
fn test_visit_bytes_does_not_match_name() {
    struct TestError;

    impl de::Error for TestError {}

    let visitor = TagOrContentVisitor {
        name: "test",
        value: PhantomData,
    };

    let result = visitor.visit_bytes(b"not_test");
    
    match result {
        Ok(TagOrContent::Content(Content::Bytes(_))) => (),
        _ => panic!("Expected TagOrContent::Content with Bytes"),
    }
}

