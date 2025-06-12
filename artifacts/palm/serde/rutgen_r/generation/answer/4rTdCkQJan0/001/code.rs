// Answer 0

#[derive(Debug)]
struct TagOrContentField {
    tag: &'static str,
    content: &'static str,
}

impl TagOrContentField {
    fn new(tag: &'static str, content: &'static str) -> Self {
        TagOrContentField { tag, content }
    }
}

mod de {
    pub trait Error {
        fn invalid_value<T>(unexpected: T, _: &super::TagOrContentField) -> Self;
    }

    #[derive(Debug)]
    pub struct MockError;

    impl Error for MockError {
        fn invalid_value<T>(_unexpected: T, _: &super::TagOrContentField) -> Self {
            MockError
        }
    }
}

impl TagOrContentField {
    fn visit_str<E>(self, field: &str) -> Result<TagOrContentField, E>
    where
        E: de::Error,
    {
        if field == self.tag {
            Ok(TagOrContentField::Tag)
        } else if field == self.content {
            Ok(TagOrContentField::Content)
        } else {
            Err(de::Error::invalid_value(field, &self))
        }
    }
}

#[test]
fn test_visit_str_tag() {
    let field_instance = TagOrContentField::new("example_tag", "example_content");
    let result: Result<TagOrContentField, MockError> = field_instance.visit_str("example_tag");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), TagOrContentField::Tag);
}

