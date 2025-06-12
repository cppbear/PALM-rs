// Answer 0

#[test]
fn test_visit_byte_buf_tag() {
    struct TestVisitor {
        name: &'static str,
    }
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContent<'de>;
        
        fn expecting(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
        
        fn visit_byte_buf<F>(self, value: Vec<u8>) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            if value == self.name.as_bytes() {
                Ok(TagOrContent::Tag)
            } else {
                Err(F::custom("Not a matching tag"))
            }
        }
    }

    let visitor = TestVisitor { name: "tag" };
    let result = visitor.visit_byte_buf(b"tag".to_vec());
    assert!(matches!(result, Ok(TagOrContent::Tag)));
}

#[test]
fn test_visit_byte_buf_content() {
    struct TestVisitor {
        name: &'static str,
    }
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContent<'de>;
        
        fn expecting(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
        
        fn visit_byte_buf<F>(self, value: Vec<u8>) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            if value == self.name.as_bytes() {
                Ok(TagOrContent::Tag)
            } else {
                ContentVisitor::new()
                    .visit_byte_buf(value)
                    .map(TagOrContent::Content)
            }
        }
    }

    let visitor = TestVisitor { name: "tag" };
    let result = visitor.visit_byte_buf(b"not a tag".to_vec());
    assert!(matches!(result, Ok(TagOrContent::Content(_))));
}

