// Answer 0

#[derive(Debug)]
struct TestStruct {
    tag: String,
    content: String,
}

impl TestStruct {
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

#[derive(Debug)]
enum TagContentOtherField {
    Tag,
    Content,
    Other,
}

#[test]
fn test_visit_bytes_tag() {
    let test_struct = TestStruct {
        tag: String::from("example_tag"),
        content: String::from("example_content"),
    };
    
    let result = test_struct.visit_bytes(b"example_tag").unwrap();
    assert_eq!(result, TagContentOtherField::Tag);
}

#[test]
fn test_visit_bytes_content() {
    let test_struct = TestStruct {
        tag: String::from("example_tag"),
        content: String::from("example_content"),
    };
    
    let result = test_struct.visit_bytes(b"example_content").unwrap();
    assert_eq!(result, TagContentOtherField::Content);
}

#[test]
fn test_visit_bytes_other() {
    let test_struct = TestStruct {
        tag: String::from("example_tag"),
        content: String::from("example_content"),
    };
    
    let result = test_struct.visit_bytes(b"other").unwrap();
    assert_eq!(result, TagContentOtherField::Other);
}

