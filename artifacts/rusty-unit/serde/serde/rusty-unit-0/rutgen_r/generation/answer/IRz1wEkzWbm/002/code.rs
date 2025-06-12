// Answer 0

#[test]
fn test_visit_u64_valid_case_field_index_1() {
    struct TestError;

    impl serde::de::Error for TestError {
        fn custom<T: std::fmt::Display>(_msg: T) -> Self {
            TestError
        }
    }

    struct TestVisitor;

    impl TestVisitor {
        fn visit_u64<E>(self, field_index: u64) -> Result<TagOrContentField, E>
        where
            E: serde::de::Error,
        {
            match field_index {
                0 => Ok(TagOrContentField::Tag),
                1 => Ok(TagOrContentField::Content),
                _ => Err(E::invalid_value(
                    serde::de::Unexpected::Unsigned(field_index),
                    &self,
                )),
            }
        }
    }

    let visitor = TestVisitor;

    let result = visitor.visit_u64::<TestError>(1);
    assert_eq!(result, Ok(TagOrContentField::Content));
}

#[test]
fn test_visit_u64_invalid_case_field_index_2() {
    struct TestError;

    impl serde::de::Error for TestError {
        fn custom<T: std::fmt::Display>(_msg: T) -> Self {
            TestError
        }
    }

    struct TestVisitor;

    impl TestVisitor {
        fn visit_u64<E>(self, field_index: u64) -> Result<TagOrContentField, E>
        where
            E: serde::de::Error,
        {
            match field_index {
                0 => Ok(TagOrContentField::Tag),
                1 => Ok(TagOrContentField::Content),
                _ => Err(E::invalid_value(
                    serde::de::Unexpected::Unsigned(field_index),
                    &self,
                )),
            }
        }
    }

    let visitor = TestVisitor;

    let result = visitor.visit_u64::<TestError>(2);
    assert!(result.is_err());
}

