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
    assert!(std::ptr::eq(&deserializer, &result));
}

