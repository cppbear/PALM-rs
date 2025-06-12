// Answer 0

#[derive(Debug)]
enum TagOrContentField {
    Tag,
    Content,
}

mod de {
    pub trait Error {
        fn invalid_value<U>(unexpected: U, field: &str) -> Self;
    }

    pub struct ErrorImpl;

    impl Error for ErrorImpl {
        fn invalid_value<U>(_unexpected: U, _field: &str) -> Self {
            ErrorImpl
        }
    }
}

struct MyVisitor {
    tag: &'static str,
    content: &'static str,
}

impl MyVisitor {
    fn visit_str<E>(&self, field: &str) -> Result<TagOrContentField, E>
    where
        E: de::Error,
    {
        if field == self.tag {
            Ok(TagOrContentField::Tag)
        } else if field == self.content {
            Ok(TagOrContentField::Content)
        } else {
            Err(de::Error::invalid_value(field, "MyVisitor"))
        }
    }
}

#[test]
fn test_visit_str_content() {
    let visitor = MyVisitor {
        tag: "not_content",
        content: "content_value",
    };
    let result: Result<TagOrContentField, de::ErrorImpl> = visitor.visit_str("content_value");
    assert_eq!(result, Ok(TagOrContentField::Content));
}

