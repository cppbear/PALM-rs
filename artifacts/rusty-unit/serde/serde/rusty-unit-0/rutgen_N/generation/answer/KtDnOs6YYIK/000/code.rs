// Answer 0

#[derive(Debug)]
enum TagContentOtherField {
    Tag,
    Content,
    Other,
}

mod de {
    pub trait Error {}
    #[derive(Debug)]
    pub struct MockError;
    impl Error for MockError {}
}

struct Visitor;

impl Visitor {
    fn visit_u64<E>(self, field_index: u64) -> Result<TagContentOtherField, E>
    where
        E: de::Error,
    {
        match field_index {
            0 => Ok(TagContentOtherField::Tag),
            1 => Ok(TagContentOtherField::Content),
            _ => Ok(TagContentOtherField::Other),
        }
    }
}

#[test]
fn test_visit_u64_tag() {
    let visitor = Visitor;
    let result = visitor.visit_u64::<de::MockError>(0);
    assert_eq!(result.unwrap(), TagContentOtherField::Tag);
}

#[test]
fn test_visit_u64_content() {
    let visitor = Visitor;
    let result = visitor.visit_u64::<de::MockError>(1);
    assert_eq!(result.unwrap(), TagContentOtherField::Content);
}

#[test]
fn test_visit_u64_other() {
    let visitor = Visitor;
    let result = visitor.visit_u64::<de::MockError>(2);
    assert_eq!(result.unwrap(), TagContentOtherField::Other);
}

