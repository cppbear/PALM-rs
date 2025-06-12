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
    #[derive(Debug)]
    pub struct Error;

    impl Error {
        pub fn invalid_value<U>(unexpected: U, _: &super::TagOrContentField) -> Result<(), Self> {
            println!("Invalid value: {:?}", unexpected);
            Err(Self {})
        }
    }
    
    pub trait ErrorTrait {
        fn invalid_value<U>(unexpected: U, field: &super::TagOrContentField) -> Result<(), Self>;
    }
    
    impl ErrorTrait for Error {
        fn invalid_value<U>(_unexpected: U, _field: &super::TagOrContentField) -> Result<(), Self> {
            Err(Error {})
        }
    }
}

impl TagOrContentField {
    fn visit_bytes<E>(&self, field: &[u8]) -> Result<TagOrContentField, E>
    where
        E: de::ErrorTrait,
    {
        if field == self.tag.as_bytes() {
            Ok(TagOrContentField::Tag)
        } else if field == self.content.as_bytes() {
            Ok(TagOrContentField::Content)
        } else {
            Err(de::Error::invalid_value(field.to_vec(), self))
        }
    }
}

#[test]
fn test_visit_bytes_invalid_value() {
    let tag_or_content = TagOrContentField::new("example_tag", "example_content");
    let result = tag_or_content.visit_bytes(b"invalid_bytes");
    assert!(result.is_err());
}

