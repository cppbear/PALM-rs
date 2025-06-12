// Answer 0

#[derive(Debug)]
struct TagOrContentField {
    tag: String,
    content: String,
}

impl TagOrContentField {
    fn new(tag: &str, content: &str) -> Self {
        TagOrContentField {
            tag: tag.to_string(),
            content: content.to_string(),
        }
    }
}

mod de {
    pub trait Error {
        fn invalid_value(value: Unexpected, expected: &str) -> Self;
    }

    #[derive(Debug)]
    pub struct Unexpected {
        bytes: Vec<u8>,
    }

    impl Unexpected {
        pub fn Bytes(bytes: &[u8]) -> Self {
            Unexpected {
                bytes: bytes.to_vec(),
            }
        }
    }
}

#[test]
fn test_visit_bytes_content() {
    struct Visitor {
        tag_or_content: TagOrContentField,
    }

    impl Visitor {
        fn visit_bytes<E>(self, field: &[u8]) -> Result<TagOrContentField, E>
        where
            E: de::Error,
        {
            if field == self.tag_or_content.tag.as_bytes() {
                Ok(TagOrContentField::Tag)
            } else if field == self.tag_or_content.content.as_bytes() {
                Ok(TagOrContentField::Content)
            } else {
                Err(de::Error::invalid_value(de::Unexpected::Bytes(field), "expected tag or content"))
            }
        }
    }

    let visitor = Visitor {
        tag_or_content: TagOrContentField::new("example_tag", "example_content"),
    };

    let result = visitor.visit_bytes(b"example_content");
    assert_eq!(result, Ok(TagOrContentField::Content));
}

