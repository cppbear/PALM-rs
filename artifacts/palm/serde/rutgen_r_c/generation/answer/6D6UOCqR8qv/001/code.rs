// Answer 0

#[test]
fn test_into_deserializer_borrowed_str() {
    use std::marker::PhantomData;

    struct MockError;
    impl serde::de::Error for MockError {
        fn custom<T>(_msg: T) -> Self {
            MockError
        }
    }

    let value = "test string";
    let deserializer = BorrowedStrDeserializer {
        value,
        marker: PhantomData::<MockError>,
    };
    let result = deserializer.into_deserializer();
    assert_eq!(result.value, value);
}

#[test]
fn test_into_deserializer_empty_str() {
    use std::marker::PhantomData;

    struct MockError;
    impl serde::de::Error for MockError {
        fn custom<T>(_msg: T) -> Self {
            MockError
        }
    }

    let value = "";
    let deserializer = BorrowedStrDeserializer {
        value,
        marker: PhantomData::<MockError>,
    };
    let result = deserializer.into_deserializer();
    assert_eq!(result.value, value);
}

