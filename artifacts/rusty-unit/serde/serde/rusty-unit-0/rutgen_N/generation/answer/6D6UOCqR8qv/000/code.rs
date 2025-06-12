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
    assert_eq!(std::ptr::addr_of!(deserializer), std::ptr::addr_of!(result));
}

