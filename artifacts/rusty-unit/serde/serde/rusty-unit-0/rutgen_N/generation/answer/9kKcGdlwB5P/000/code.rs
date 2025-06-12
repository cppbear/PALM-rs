// Answer 0

#[test]
fn test_missing_field_option() {
    use serde::de::{self, Deserializer, Visitor};
    use serde::Deserialize;
    use std::marker::PhantomData;

    #[derive(Deserialize)]
    struct TestStruct {
        field: Option<String>,
    }

    struct MissingFieldError;

    impl de::Error for MissingFieldError {
        fn custom<T>(_msg: T) -> Self {
            MissingFieldError
        }
    }

    let result: Result<TestStruct, MissingFieldError> = missing_field("field");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().field, None);
}

#[test]
fn test_missing_field_non_option() {
    use serde::de::{self, Deserializer, Visitor};
    use serde::Deserialize;
    use std::marker::PhantomData;

    #[derive(Deserialize)]
    struct TestStruct {
        field: String,
    }

    struct MissingFieldError;

    impl de::Error for MissingFieldError {
        fn custom<T>(_msg: T) -> Self {
            MissingFieldError
        }
    }

    let result: Result<TestStruct, MissingFieldError> = missing_field("field");
    assert!(result.is_err());
}

