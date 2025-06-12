// Answer 0

#[test]
fn test_into_deserializer() {
    struct TestStruct;

    impl TestStruct {
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let instance = TestStruct;
    let result = instance.into_deserializer();
    assert_eq!(std::ptr::addr_of!(result), std::ptr::addr_of!(instance));
}

#[test]
fn test_into_deserializer_with_multiple_calls() {
    struct TestStruct;

    impl TestStruct {
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let instance = TestStruct;
    let first_result = instance.into_deserializer();
    let second_result = first_result.into_deserializer();
    assert_eq!(std::ptr::addr_of!(second_result), std::ptr::addr_of!(first_result));
}

