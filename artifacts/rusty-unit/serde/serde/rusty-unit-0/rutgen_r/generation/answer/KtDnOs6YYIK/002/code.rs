// Answer 0

#[derive(Debug)]
enum TagContentOtherField {
    Tag,
    Content,
    Other,
}

mod de {
    pub trait Error {}
}

struct TestError;

impl de::Error for TestError {}

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
fn test_visit_u64_with_field_index_1() {
    let visitor = Visitor;
    let result: Result<TagContentOtherField, TestError> = visitor.visit_u64(1);
    assert_eq!(result.unwrap(), TagContentOtherField::Content);
}

