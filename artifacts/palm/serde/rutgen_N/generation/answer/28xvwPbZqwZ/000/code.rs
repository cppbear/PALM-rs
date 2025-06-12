// Answer 0

#[test]
fn test_into_deserializer() {
    struct MockDeserializer;

    impl MockDeserializer {
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let deserializer = MockDeserializer;
    let result = deserializer.into_deserializer();
    assert_eq!(std::ptr::eq(&deserializer, &result), true);
}

