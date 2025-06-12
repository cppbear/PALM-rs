// Answer 0

#[test]
fn test_into_deserializer_returns_self() {
    struct DeserializerWrapper;

    impl DeserializerWrapper {
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let deserializer = DeserializerWrapper;
    let returned = deserializer.into_deserializer();
    assert_eq!(std::ptr::addr_of!(returned), std::ptr::addr_of!(deserializer));
}

#[test]
fn test_into_deserializer_with_multiple_instances() {
    struct DeserializerWrapper;

    impl DeserializerWrapper {
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let deserializer1 = DeserializerWrapper;
    let returned1 = deserializer1.into_deserializer();
    assert_eq!(std::ptr::addr_of!(returned1), std::ptr::addr_of!(deserializer1));

    let deserializer2 = DeserializerWrapper;
    let returned2 = deserializer2.into_deserializer();
    assert_eq!(std::ptr::addr_of!(returned2), std::ptr::addr_of!(deserializer2));
}

