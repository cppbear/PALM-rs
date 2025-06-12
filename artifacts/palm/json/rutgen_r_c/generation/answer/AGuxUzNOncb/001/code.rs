// Answer 0

#[test]
fn test_into_deserializer() {
    struct RawValue;

    impl RawValue {
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let raw_value = RawValue;
    let deserializer = raw_value.into_deserializer();
    assert_eq!(std::ptr::addr_of!(deserializer), std::ptr::addr_of!(raw_value));
}

#[test]
#[should_panic]
fn test_into_deserializer_panic() {
    struct RawValue;

    impl RawValue {
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let raw_value = RawValue;
    let deserializer = raw_value.into_deserializer();
    drop(raw_value); // Ensure panic on subsequent access
    let _ = deserializer.into_deserializer(); // Attempting to use deserializer after raw_value is dropped
}

