// Answer 0

#[test]
fn test_unit_deserializer_new() {
    use std::marker::PhantomData;

    struct TestError;
    impl serde::de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    let unit_deserializer: UnitDeserializer<TestError> = UnitDeserializer::new();
    assert!(unit_deserializer.marker == PhantomData::<TestError>);
}

