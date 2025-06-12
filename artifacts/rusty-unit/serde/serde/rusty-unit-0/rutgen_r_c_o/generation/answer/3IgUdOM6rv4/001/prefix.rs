// Answer 0

#[test]
fn test_into_deserializer_with_unit_deserializer() {
    struct TestStruct;
    impl IntoDeserializer<'static> for TestStruct {
        type Deserializer = UnitDeserializer<Error>;
        fn into_deserializer(self) -> Self::Deserializer {
            UnitDeserializer::new()
        }
    }

    let test_instance = TestStruct;
    let _deserializer: UnitDeserializer<Error> = test_instance.into_deserializer();
}

#[test]
fn test_into_deserializer_with_another_instance() {
    struct AnotherTestStruct;
    impl IntoDeserializer<'static> for AnotherTestStruct {
        type Deserializer = UnitDeserializer<Error>;
        fn into_deserializer(self) -> Self::Deserializer {
            UnitDeserializer::new()
        }
    }

    let another_test_instance = AnotherTestStruct;
    let _deserializer: UnitDeserializer<Error> = another_test_instance.into_deserializer();
}

