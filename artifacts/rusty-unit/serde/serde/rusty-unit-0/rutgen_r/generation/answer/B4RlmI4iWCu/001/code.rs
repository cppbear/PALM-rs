// Answer 0

#[test]
fn test_visit_bool_true() {
    struct MockError;

    impl de::Error for MockError {}

    struct MockVisitor;

    impl MockVisitor {
        fn new() -> Self {
            MockVisitor
        }

        fn visit_bool(&self, value: bool) -> Result<(), MockError> {
            if value {
                Ok(())
            } else {
                Err(MockError)
            }
        }
    }

    struct ContentVisitor;

    impl ContentVisitor {
        fn new() -> Self {
            ContentVisitor
        }

        fn visit_bool(self, value: bool) -> Result<ContentVisitor, MockError> {
            MockVisitor::new().visit_bool(value)?;
            Ok(self)
        }
    }

    enum TagOrContent {
        Content(ContentVisitor),
    }

    struct TestStruct;

    impl TestStruct {
        fn visit_bool<F>(self, value: bool) -> Result<TagOrContent, F>
        where
            F: de::Error,
        {
            ContentVisitor::new()
                .visit_bool(value)
                .map(TagOrContent::Content)
        }
    }

    let test_struct = TestStruct;

    let result = test_struct.visit_bool(true);
    assert!(result.is_ok());
}

#[test]
fn test_visit_bool_false() {
    struct MockError;

    impl de::Error for MockError {}

    struct MockVisitor;

    impl MockVisitor {
        fn new() -> Self {
            MockVisitor
        }

        fn visit_bool(&self, value: bool) -> Result<(), MockError> {
            if value {
                Ok(())
            } else {
                Err(MockError)
            }
        }
    }

    struct ContentVisitor;

    impl ContentVisitor {
        fn new() -> Self {
            ContentVisitor
        }

        fn visit_bool(self, value: bool) -> Result<ContentVisitor, MockError> {
            MockVisitor::new().visit_bool(value)?;
            Ok(self)
        }
    }

    enum TagOrContent {
        Content(ContentVisitor),
    }

    struct TestStruct;

    impl TestStruct {
        fn visit_bool<F>(self, value: bool) -> Result<TagOrContent, F>
        where
            F: de::Error,
        {
            ContentVisitor::new()
                .visit_bool(value)
                .map(TagOrContent::Content)
        }
    }

    let test_struct = TestStruct;

    let result = test_struct.visit_bool(false);
    assert!(result.is_err());
}

