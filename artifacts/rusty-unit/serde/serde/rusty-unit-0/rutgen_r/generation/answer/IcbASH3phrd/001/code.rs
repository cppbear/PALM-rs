// Answer 0

#[test]
fn test_into_deserializer() {
    struct TestStruct;

    impl TestStruct {
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let test_instance = TestStruct;
    let result = test_instance.into_deserializer();
    assert_eq!(std::ptr::addr_of!(result), std::ptr::addr_of!(test_instance));
}

#[test]
fn test_into_deserializer_empty() {
    struct EmptyStruct;

    impl EmptyStruct {
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let empty_instance = EmptyStruct;
    let result = empty_instance.into_deserializer();
    assert_eq!(std::ptr::addr_of!(result), std::ptr::addr_of!(empty_instance));
}

