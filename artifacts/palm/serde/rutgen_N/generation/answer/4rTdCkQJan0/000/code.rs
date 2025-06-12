// Answer 0

#[derive(Debug)]
struct TagOrContentField {
    tag: &'static str,
    content: &'static str,
}

impl TagOrContentField {
    const Tag: &'static str = "tag";
    const Content: &'static str = "content";
}

mod de {
    pub trait Error {
        fn invalid_value(unexpected: Unexpected, _: &super::TagOrContentField) -> Self;
    }

    #[derive(Debug)]
    pub struct Unexpected {
        value: String,
    }

    impl Unexpected {
        pub fn str(value: &str) -> Self {
            Unexpected {
                value: value.to_string(),
            }
        }
    }
}

impl TagOrContentField {
    fn visit_str<E>(self, field: &str) -> Result<&'static str, E>
    where
        E: de::Error,
    {
        if field == self.tag {
            Ok(TagOrContentField::Tag)
        } else if field == self.content {
            Ok(TagOrContentField::Content)
        } else {
            Err(de::Error::invalid_value(de::Unexpected::str(field), &self))
        }
    }
}

#[derive(Debug)]
struct TestError;

impl de::Error for TestError {
    fn invalid_value(_: de::Unexpected, _: &TagOrContentField) -> Self {
        TestError
    }
}

#[test]
fn test_visit_str_tag() {
    let field = TagOrContentField {
        tag: "tag",
        content: "content",
    };
    let result = field.visit_str("tag");
    assert_eq!(result, Ok(TagOrContentField::Tag));
}

#[test]
fn test_visit_str_content() {
    let field = TagOrContentField {
        tag: "tag",
        content: "content",
    };
    let result = field.visit_str("content");
    assert_eq!(result, Ok(TagOrContentField::Content));
}

#[test]
fn test_visit_str_invalid() {
    let field = TagOrContentField {
        tag: "tag",
        content: "content",
    };
    let result: Result<&'static str, TestError> = field.visit_str("invalid");
    assert!(result.is_err());
}

