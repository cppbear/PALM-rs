// Answer 0

#[test]
fn test_into_deserializer() {
    struct TestDeserializer;

    impl TestDeserializer {
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let deserializer = TestDeserializer;
    let result = deserializer.into_deserializer();
    
    assert_eq!(std::ptr::eq(&deserializer, &result), true);
}

