// Answer 0

#[cfg(test)]
mod tests {
    use serde::de;
    use serde::private::ContentVisitor;
    use serde::private::TagOrContent;

    struct TestError;
    
    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }
    
    struct TestVisitor {
        name: String,
    }

    impl TestVisitor {
        fn new(name: &str) -> Self {
            TestVisitor { name: name.to_string() }
        }

        fn visit_borrowed_bytes<F>(&self, value: &'static [u8]) -> Result<TagOrContent, F>
        where
            F: de::Error,
        {
            if value == self.name.as_bytes() {
                Ok(TagOrContent::Tag)
            } else {
                ContentVisitor::new()
                    .visit_borrowed_bytes(value)
                    .map(TagOrContent::Content)
            }
        }
    }
    
    #[test]
    fn test_visit_borrowed_bytes_content() {
        let visitor = TestVisitor::new("expected_name");
        let input_value: &'static [u8] = b"unexpected_value"; // This does not match "expected_name"

        let result: Result<TagOrContent, TestError> = visitor.visit_borrowed_bytes(input_value);

        assert!(result.is_ok());
        if let Ok(tag_or_content) = result {
            match tag_or_content {
                TagOrContent::Content => {},
                _ => panic!("Expected Content, but got Tag"),
            }
        }
    }
    
    #[test]
    #[should_panic]
    fn test_visit_borrowed_bytes_panic() {
        let visitor = TestVisitor::new("expected_name");
        let input_value: &'static [u8] = b"expected_name"; // This matches "expected_name", testing for a panic condition

        let result: Result<TagOrContent, TestError> = visitor.visit_borrowed_bytes(input_value);
        
        assert!(result.is_ok());
        if let Ok(tag_or_content) = result {
            match tag_or_content {
                TagOrContent::Tag => {},
                _ => panic!("Expected Tag, but got Content"),
            }
        }
    }
}

