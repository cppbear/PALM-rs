// Answer 0

#[test]
fn test_visit_f64() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    struct TestVisitor;

    impl TestVisitor {
        fn new() -> Self {
            TestVisitor
        }

        fn visit_f64(self, value: f64) -> Result<(), TestError> {
            if value.is_finite() {
                Ok(())
            } else {
                Err(TestError)
            }
        }
    }

    struct ContentVisitor;

    impl ContentVisitor {
        fn new() -> Self {
            ContentVisitor
        }

        fn visit_f64(self, value: f64) -> Result<TagOrContent, TestError> {
            TestVisitor::new().visit_f64(value)?;
            Ok(TagOrContent::Content)
        }
    }

    enum TagOrContent {
        Content,
    }

    // Test for a finite f64 value
    let result = ContentVisitor::new().visit_f64(1.0);
    assert!(result.is_ok());

    // Test for zero
    let result = ContentVisitor::new().visit_f64(0.0);
    assert!(result.is_ok());

    // Test for a negative finite value
    let result = ContentVisitor::new().visit_f64(-1.0);
    assert!(result.is_ok());

    // Test for NaN
    let result = ContentVisitor::new().visit_f64(f64::NAN);
    assert!(result.is_err());

    // Test for positive infinity
    let result = ContentVisitor::new().visit_f64(f64::INFINITY);
    assert!(result.is_err());

    // Test for negative infinity
    let result = ContentVisitor::new().visit_f64(f64::NEG_INFINITY);
    assert!(result.is_err());
}

