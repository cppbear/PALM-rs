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

struct TestVisitor;

impl TestVisitor {
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
    let visitor = TestVisitor;
    let result = visitor.visit_u64::<TestError>(0);
    assert_eq!(result, Ok(TagContentOtherField::Tag));
}

#[test]
fn test_visit_u64_content() {
    let visitor = TestVisitor;
    let result = visitor.visit_u64::<TestError>(1);
    assert_eq!(result, Ok(TagContentOtherField::Content));
}

#[test]
fn test_visit_u64_other() {
    let visitor = TestVisitor;
    let result = visitor.visit_u64::<TestError>(2);
    assert_eq!(result, Ok(TagContentOtherField::Other));
}

#[test]
fn test_visit_u64_other_boundary() {
    let visitor = TestVisitor;
    let result = visitor.visit_u64::<TestError>(u64::MAX);
    assert_eq!(result, Ok(TagContentOtherField::Other));
}

