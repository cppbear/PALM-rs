// Answer 0

#[test]
fn test_visit_bytes_tag() {
    struct TestVisitor {
        tag: &'static str,
        content: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagContentOtherField;

        fn expecting(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
        
        fn visit_bytes<E>(self, field: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if field == self.tag.as_bytes() {
                Ok(TagContentOtherField::Tag)
            } else if field == self.content.as_bytes() {
                Ok(TagContentOtherField::Content)
            } else {
                Ok(TagContentOtherField::Other)
            }
        }
    }

    let visitor = TestVisitor {
        tag: "tag_field",
        content: "content_field",
    };

    let result = visitor.visit_str("tag_field").unwrap();
    assert_eq!(result, TagContentOtherField::Tag);
}

#[test]
fn test_visit_bytes_content() {
    struct TestVisitor {
        tag: &'static str,
        content: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagContentOtherField;

        fn expecting(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
        
        fn visit_bytes<E>(self, field: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if field == self.tag.as_bytes() {
                Ok(TagContentOtherField::Tag)
            } else if field == self.content.as_bytes() {
                Ok(TagContentOtherField::Content)
            } else {
                Ok(TagContentOtherField::Other)
            }
        }
    }

    let visitor = TestVisitor {
        tag: "tag_field",
        content: "content_field",
    };

    let result = visitor.visit_str("content_field").unwrap();
    assert_eq!(result, TagContentOtherField::Content);
}

#[test]
fn test_visit_bytes_other() {
    struct TestVisitor {
        tag: &'static str,
        content: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagContentOtherField;

        fn expecting(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
        
        fn visit_bytes<E>(self, field: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if field == self.tag.as_bytes() {
                Ok(TagContentOtherField::Tag)
            } else if field == self.content.as_bytes() {
                Ok(TagContentOtherField::Content)
            } else {
                Ok(TagContentOtherField::Other)
            }
        }
    }

    let visitor = TestVisitor {
        tag: "tag_field",
        content: "content_field",
    };

    let result = visitor.visit_str("other_field").unwrap();
    assert_eq!(result, TagContentOtherField::Other);
}

