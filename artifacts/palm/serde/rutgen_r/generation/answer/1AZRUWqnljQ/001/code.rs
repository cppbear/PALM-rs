// Answer 0

#[derive(Debug)]
struct TagOrContentField {
    tag: String,
    content: String,
}

mod de {
    pub trait Error {
        fn invalid_value<T>(unexpected: Unexpected, _: &T) -> Self;
    }

    #[derive(Debug)]
    pub enum Unexpected {
        Bytes(Vec<u8>),
    }
}

impl TagOrContentField {
    fn visit_bytes<E>(&self, field: &[u8]) -> Result<TagOrContentField, E>
    where
        E: de::Error,
    {
        if field == self.tag.as_bytes() {
            Ok(TagOrContentField::Tag)
        } else if field == self.content.as_bytes() {
            Ok(TagOrContentField::Content)
        } else {
            Err(de::Error::invalid_value(de::Unexpected::Bytes(field.to_vec()), &self))
        }
    }
}

#[derive(Debug)]
struct MockError;

impl de::Error for MockError {
    fn invalid_value<T>(unexpected: de::Unexpected, _: &T) -> Self {
        panic!("{:?}", unexpected);
    }
}

#[test]
fn test_visit_bytes_with_matching_tag() {
    let tag_or_content = TagOrContentField {
        tag: "tag".to_string(),
        content: "content".to_string(),
    };
    let result = tag_or_content.visit_bytes(b"tag");
    assert_eq!(result, Ok(TagOrContentField::Tag));
}

