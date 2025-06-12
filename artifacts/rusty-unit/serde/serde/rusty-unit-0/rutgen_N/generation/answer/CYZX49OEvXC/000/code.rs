// Answer 0

#[derive(Debug, PartialEq)]
enum TagContentOtherField {
    Tag,
    Content,
    Other,
}

struct TestDeserializer {
    tag: String,
    content: String,
}

impl TestDeserializer {
    fn new(tag: &str, content: &str) -> Self {
        TestDeserializer {
            tag: tag.to_string(),
            content: content.to_string(),
        }
    }
    
    fn visit_bytes<E>(self, field: &[u8]) -> Result<TagContentOtherField, E>
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

#[test]
fn test_visit_bytes_tag() {
    let deserializer = TestDeserializer::new("example_tag", "example_content");
    let result = deserializer.visit_bytes(b"example_tag").unwrap();
    assert_eq!(result, TagContentOtherField::Tag);
}

#[test]
fn test_visit_bytes_content() {
    let deserializer = TestDeserializer::new("example_tag", "example_content");
    let result = deserializer.visit_bytes(b"example_content").unwrap();
    assert_eq!(result, TagContentOtherField::Content);
}

#[test]
fn test_visit_bytes_other() {
    let deserializer = TestDeserializer::new("example_tag", "example_content");
    let result = deserializer.visit_bytes(b"other_field").unwrap();
    assert_eq!(result, TagContentOtherField::Other);
}

