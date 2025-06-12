// Answer 0

#[test]
fn test_into_deserializer_return_value() {
    struct Deserializer;

    impl Deserializer {
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let deserializer = Deserializer;
    let result = deserializer.into_deserializer();
    assert_eq!(std::ptr::eq(&result, &deserializer), true);
}

#[test]
fn test_into_deserializer_with_multiple_instances() {
    struct Deserializer;

    impl Deserializer {
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let deserializer1 = Deserializer;
    let deserializer2 = deserializer1.into_deserializer();
    assert_eq!(std::ptr::eq(&deserializer2, &deserializer1), true);
}

