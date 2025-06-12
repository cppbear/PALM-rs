// Answer 0

#[test]
fn test_into_deserializer() {
    struct MyDeserializer;

    impl MyDeserializer {
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let deserializer = MyDeserializer;
    let result = deserializer.into_deserializer();
    assert_eq!(std::ptr::eq(&deserializer, &result), true);
}

