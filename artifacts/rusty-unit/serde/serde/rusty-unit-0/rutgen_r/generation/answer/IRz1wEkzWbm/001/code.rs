// Answer 0

#[derive(Debug)]
enum TagOrContentField {
    Tag,
    Content,
}

mod de {
    #[derive(Debug)]
    pub struct Error;

    impl Error {
        pub fn invalid_value<T>(unexpected: Unexpected, _: &T) -> Self {
            println!("{:?}", unexpected);
            Error
        }
    }

    #[derive(Debug)]
    pub struct Unexpected {
        value: u64,
    }

    impl Unexpected {
        pub fn Unsigned(value: u64) -> Self {
            Unexpected { value }
        }
    }
}

struct Tester;

impl Tester {
    fn visit_u64<E>(self, field_index: u64) -> Result<TagOrContentField, E>
    where
        E: de::Error,
    {
        match field_index {
            0 => Ok(TagOrContentField::Tag),
            1 => Ok(TagOrContentField::Content),
            _ => Err(de::Error::invalid_value(
                de::Unexpected::Unsigned(field_index),
                &self,
            )),
        }
    }
}

#[test]
fn test_visit_u64_valid_tag() {
    let tester = Tester;
    assert_eq!(tester.visit_u64::<de::Error>(0), Ok(TagOrContentField::Tag));
}

#[test]
fn test_visit_u64_valid_content() {
    let tester = Tester;
    assert_eq!(tester.visit_u64::<de::Error>(1), Ok(TagOrContentField::Content));
}

#[test]
#[should_panic]
fn test_visit_u64_invalid_value() {
    let tester = Tester;
    let result: Result<TagOrContentField, de::Error> = tester.visit_u64(2);
    assert!(result.is_err());
}

