// Answer 0

#[derive(Debug)]
enum TagOrContentField {
    Tag,
    Content,
}

mod de {
    pub trait Error {
        fn invalid_value<T>(unexpected: T, expected: &str) -> Self;
    }
}

struct TestError;

impl de::Error for TestError {
    fn invalid_value<T>(_unexpected: T, _expected: &str) -> Self {
        TestError
    }
}

struct Tester;

impl Tester {
    fn visit_u64<E>(&self, field_index: u64) -> Result<TagOrContentField, E>
    where
        E: de::Error,
    {
        match field_index {
            0 => Ok(TagOrContentField::Tag),
            1 => Ok(TagOrContentField::Content),
            _ => Err(E::invalid_value(field_index, "0 or 1")),
        }
    }
}

#[test]
fn test_visit_u64_returns_tag_for_zero() {
    let tester = Tester;
    let result: Result<TagOrContentField, TestError> = tester.visit_u64(0);
    assert_eq!(result, Ok(TagOrContentField::Tag));
}

#[test]
fn test_visit_u64_returns_content_for_one() {
    let tester = Tester;
    let result: Result<TagOrContentField, TestError> = tester.visit_u64(1);
    assert_eq!(result, Ok(TagOrContentField::Content));
}

#[test]
fn test_visit_u64_returns_error_for_invalid_index() {
    let tester = Tester;
    let result: Result<TagOrContentField, TestError> = tester.visit_u64(2);
    assert!(result.is_err());
}

