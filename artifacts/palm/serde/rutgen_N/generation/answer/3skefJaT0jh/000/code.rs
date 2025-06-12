// Answer 0

#[test]
fn test_visit_i64_success() {
    struct TestError;
    impl de::Error for TestError {}
    
    struct TestVisitor;

    impl TestVisitor {
        fn new() -> Self {
            TestVisitor
        }

        fn visit_i64(self, value: i64) -> Result<i64, TestError> {
            Ok(value)
        }
    }

    let visitor = TestVisitor::new();
    let result: Result<TagOrContent<i64>, TestError> = visitor.visit_i64(42)
        .map(TagOrContent::Content);
    
    assert_eq!(result.unwrap(), TagOrContent::Content(42));
}

#[test]
#[should_panic]
fn test_visit_i64_failure() {
    struct TestError;
    impl de::Error for TestError {}

    struct TestVisitor;

    impl TestVisitor {
        fn new() -> Self {
            TestVisitor
        }

        fn visit_i64(self, value: i64) -> Result<i64, TestError> {
            Err(TestError)
        }
    }

    let visitor = TestVisitor::new();
    let _result: Result<TagOrContent<i64>, TestError> = visitor.visit_i64(42)
        .map(TagOrContent::Content)
        .expect("Expected an error but got success");
}

