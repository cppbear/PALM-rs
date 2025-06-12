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
    assert_eq!(std::ptr::eq(&instance, &result), false); // Since it should return a new instance by value
}

#[test]
fn test_into_deserializer_with_clone() {
    #[derive(Clone)]
    struct TestCloneStruct;

    impl TestCloneStruct {
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let instance = TestCloneStruct;
    let result = instance.clone().into_deserializer();
    assert_eq!(std::ptr::eq(&instance, &result), false); // Since it should return a new instance by value
}

