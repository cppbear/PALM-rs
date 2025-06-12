// Answer 0

#[test]
fn test_expecting() {
    use std::fmt;

    struct MockFormatter;

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let visitor = TagOrContentFieldVisitor {
        tag: "tag_field",
        content: "content_field",
    };

    let mut mock_formatter = MockFormatter;

    let result = visitor.expecting(&mut mock_formatter);

    assert!(result.is_ok());
}

#[test]
fn test_visit_u64_tag() {
    struct MockError;
    struct MockVisitor {
        tag: &'static str,
        content: &'static str,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = TagOrContentField;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "{} or {}", self.tag, self.content)
        }

        fn visit_u64<E>(self, field_index: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match field_index {
                0 => Ok(TagOrContentField::Tag),
                _ => Err(de::Error::invalid_value(Unexpected::Unsigned(field_index), &self)),
            }
        }
    }

    let visitor = MockVisitor {
        tag: "tag_field",
        content: "content_field",
    };

    assert_eq!(visitor.visit_u64(0), Ok(TagOrContentField::Tag));
}

#[test]
fn test_visit_u64_content() {
    struct MockError;
    struct MockVisitor {
        tag: &'static str,
        content: &'static str,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = TagOrContentField;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "{} or {}", self.tag, self.content)
        }

        fn visit_u64<E>(self, field_index: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match field_index {
                1 => Ok(TagOrContentField::Content),
                _ => Err(de::Error::invalid_value(Unexpected::Unsigned(field_index), &self)),
            }
        }
    }

    let visitor = MockVisitor {
        tag: "tag_field",
        content: "content_field",
    };

    assert_eq!(visitor.visit_u64(1), Ok(TagOrContentField::Content));
}

#[test]
fn test_visit_u64_invalid() {
    struct MockError;
    struct MockVisitor {
        tag: &'static str,
        content: &'static str,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = TagOrContentField;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "{} or {}", self.tag, self.content)
        }

        fn visit_u64<E>(self, field_index: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match field_index {
                0 => Ok(TagOrContentField::Tag),
                1 => Ok(TagOrContentField::Content),
                _ => Err(de::Error::invalid_value(Unexpected::Unsigned(field_index), &self)),
            }
        }
    }

    let visitor = MockVisitor {
        tag: "tag_field",
        content: "content_field",
    };

    let result = visitor.visit_u64(2);
    assert!(result.is_err());
}

