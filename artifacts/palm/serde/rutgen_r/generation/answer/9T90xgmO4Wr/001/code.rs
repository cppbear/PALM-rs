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
    assert_eq!(std::mem::discriminant(&result), std::mem::discriminant(&test_instance));
}

#[test]
fn test_into_deserializer_panic() {
    struct PanicStruct;

    impl PanicStruct {
        fn into_deserializer(self) -> Self {
            panic!("Intentional panic for testing");
        }
    }

    let panic_instance = PanicStruct;
    let result = std::panic::catch_unwind(|| {
        panic_instance.into_deserializer();
    });

    assert!(result.is_err());
}

