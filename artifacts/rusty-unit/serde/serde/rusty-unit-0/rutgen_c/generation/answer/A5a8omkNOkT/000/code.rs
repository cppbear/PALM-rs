// Answer 0

#[test]
fn test_visit_str_tag() {
    struct TestVisitor {
        tag: &'static str,
        content: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagContentOtherField;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "Expecting tag or content field")
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

    let result = visitor.visit_str("tag_field");
    assert_eq!(result.unwrap(), TagContentOtherField::Tag);
}

#[test]
fn test_visit_str_content() {
    struct TestVisitor {
        tag: &'static str,
        content: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagContentOtherField;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "Expecting tag or content field")
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

    let result = visitor.visit_str("content_field");
    assert_eq!(result.unwrap(), TagContentOtherField::Content);
}

#[test]
fn test_visit_str_other() {
    struct TestVisitor {
        tag: &'static str,
        content: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagContentOtherField;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "Expecting tag or content field")
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

    let result = visitor.visit_str("unknown_field");
    assert_eq!(result.unwrap(), TagContentOtherField::Other);
}

