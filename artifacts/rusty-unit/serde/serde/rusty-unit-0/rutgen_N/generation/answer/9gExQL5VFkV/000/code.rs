// Answer 0

#[test]
fn test_borrowed_str_deserializer_new() {
    use serde::de::value::BorrowedStrDeserializer;
    use std::marker::PhantomData;

    struct DummyError;

    // Dummy type to satisfy the generic parameter E
    struct DummyDeserializer<'de> {
        value: &'de str,
        marker: PhantomData<&'de ()>,
    }

    impl<'de> BorrowedStrDeserializer<'de, DummyError> {
        pub fn new(value: &'de str) -> Self {
            BorrowedStrDeserializer {
                value,
                marker: PhantomData,
            }
        }
    }

    let input_value = "test string";
    let deserializer: BorrowedStrDeserializer<DummyError> = DummyDeserializer::new(input_value);
    assert_eq!(deserializer.value, input_value);
}

