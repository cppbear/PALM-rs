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

#[derive(Debug)]
struct MyError;

impl de::Error for MyError {}

struct Visitor;

impl Visitor {
    fn visit_u64<E>(&self, field_index: u64) -> Result<TagContentOtherField, E>
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
fn test_visit_u64_case_0() {
    let visitor = Visitor;
    let result: Result<TagContentOtherField, MyError> = visitor.visit_u64(0);
    assert_eq!(result.unwrap(), TagContentOtherField::Tag);
}

#[test]
fn test_visit_u64_case_1() {
    let visitor = Visitor;
    let result: Result<TagContentOtherField, MyError> = visitor.visit_u64(1);
    assert_eq!(result.unwrap(), TagContentOtherField::Content);
}

#[test]
fn test_visit_u64_case_other() {
    let visitor = Visitor;
    let result: Result<TagContentOtherField, MyError> = visitor.visit_u64(2);
    assert_eq!(result.unwrap(), TagContentOtherField::Other);
}

