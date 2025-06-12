// Answer 0

#[test]
fn test_into_deserializer_returns_unit_deserializer() {
    struct TestValue;

    impl IntoDeserializer<'static> for TestValue {
        type Deserializer = UnitDeserializer<Error>;

        fn into_deserializer(self) -> Self::Deserializer {
            UnitDeserializer::new()
        }
    }

    let value = TestValue;
    let deserializer: UnitDeserializer<Error> = value.into_deserializer();
    assert!(std::mem::size_of_val(&deserializer) > 0);
}

#[test]
fn test_into_deserializer_creates_different_instances() {
    struct TestValue;

    impl IntoDeserializer<'static> for TestValue {
        type Deserializer = UnitDeserializer<Error>;

        fn into_deserializer(self) -> Self::Deserializer {
            UnitDeserializer::new()
        }
    }

    let value1 = TestValue;
    let deserializer1: UnitDeserializer<Error> = value1.into_deserializer();

    let value2 = TestValue;
    let deserializer2: UnitDeserializer<Error> = value2.into_deserializer();

    assert_ne!(&deserializer1 as *const _, &deserializer2 as *const _);
}

