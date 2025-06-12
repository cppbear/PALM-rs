// Answer 0

#[test]
fn test_visit_bytes_content() {
    struct TestVisitor {
        tag: &'static str,
        content: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContentField;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("expecting a valid field")
        }

        fn visit_bytes<E>(self, field: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if field == self.tag.as_bytes() {
                Ok(TagOrContentField::Tag)
            } else if field == self.content.as_bytes() {
                Ok(TagOrContentField::Content)
            } else {
                Err(de::Error::invalid_value(Unexpected::Bytes(field), &self))
            }
        }
    }

    let visitor = TestVisitor {
        tag: "example_tag",
        content: "example_content",
    };

    let input: &[u8] = b"example_content";
    let _ = visitor.visit_bytes(input);
}

#[test]
fn test_visit_bytes_content_different_length() {
    struct TestVisitor {
        tag: &'static str,
        content: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContentField;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("expecting a valid field")
        }

        fn visit_bytes<E>(self, field: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if field == self.tag.as_bytes() {
                Ok(TagOrContentField::Tag)
            } else if field == self.content.as_bytes() {
                Ok(TagOrContentField::Content)
            } else {
                Err(de::Error::invalid_value(Unexpected::Bytes(field), &self))
            }
        }
    }

    let visitor = TestVisitor {
        tag: "tag",
        content: "content",
    };

    let input: &[u8] = b"content";
    let _ = visitor.visit_bytes(input);
}

#[test]
fn test_visit_bytes_content_with_extra_bytes() {
    struct TestVisitor {
        tag: &'static str,
        content: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContentField;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("expecting a valid field")
        }

        fn visit_bytes<E>(self, field: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if field == self.tag.as_bytes() {
                Ok(TagOrContentField::Tag)
            } else if field == self.content.as_bytes() {
                Ok(TagOrContentField::Content)
            } else {
                Err(de::Error::invalid_value(Unexpected::Bytes(field), &self))
            }
        }
    }

    let visitor = TestVisitor {
        tag: "tag",
        content: "content",
    };

    let input: &[u8] = b"content_extra";
    let _ = visitor.visit_bytes(input);
}

