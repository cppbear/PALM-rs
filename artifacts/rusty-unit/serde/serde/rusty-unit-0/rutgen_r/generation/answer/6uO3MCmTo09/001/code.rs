// Answer 0

#[test]
fn test_visit_bytes_tag() {
    struct TestError;

    impl de::Error for TestError {}

    struct TestVisitor {
        name: String,
    }

    impl TestVisitor {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
            }
        }
        
        fn visit_bytes<F>(self, value: &[u8]) -> Result<TagOrContent, F>
        where
            F: de::Error,
        {
            if value == self.name.as_bytes() {
                Ok(TagOrContent::Tag)
            } else {
                ContentVisitor::new()
                    .visit_bytes(value)
                    .map(TagOrContent::Content)
            }
        }
    }

    enum TagOrContent {
        Tag,
        Content,
    }

    struct ContentVisitor;

    impl ContentVisitor {
        fn new() -> Self {
            ContentVisitor
        }
        
        fn visit_bytes<F>(&self, _value: &[u8]) -> Result<(), F>
        where
            F: de::Error,
        {
            Ok(())
        }
    }

    let visitor = TestVisitor::new("test_name");
    let result = visitor.visit_bytes(b"test_name");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), TagOrContent::Tag);
}

