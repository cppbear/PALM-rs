// Answer 0

#[test]
fn test_into_deserializer() {
    struct Deserializer;

    impl Deserializer {
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let deserializer = Deserializer;
    let result = deserializer.into_deserializer();
    assert_eq!(std::ptr::addr_of!(result), std::ptr::addr_of!(deserializer));
}

#[test]
fn test_into_deserializer_with_alternate_data() {
    struct AlternateDeserializer;

    impl AlternateDeserializer {
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let alternate_deserializer = AlternateDeserializer;
    let result = alternate_deserializer.into_deserializer();
    assert_eq!(std::ptr::addr_of!(result), std::ptr::addr_of!(alternate_deserializer));
}

