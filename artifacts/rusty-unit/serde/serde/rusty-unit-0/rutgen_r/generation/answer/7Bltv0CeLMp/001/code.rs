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
    assert!(std::ptr::eq(&result, &deserializer));
}

#[test]
#[should_panic]
fn test_into_deserializer_panic() {
    struct TestStruct;

    impl TestStruct {
        fn into_deserializer(self) -> Self {
            // Here we can create a situation that leads to panic if we had mutable states.
            panic!("deliberate panic for testing");
        }
    }

    let instance = TestStruct;
    instance.into_deserializer();
}

