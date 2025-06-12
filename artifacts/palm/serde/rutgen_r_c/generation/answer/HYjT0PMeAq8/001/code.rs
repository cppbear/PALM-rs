// Answer 0

#[test]
fn test_into_deserializer() {
    #[derive(Clone, Debug)]
    struct TestDeserializer;

    impl<'de> IntoDeserializer<'de> for TestDeserializer {
        type Deserializer = Self;
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let deserializer = TestDeserializer;
    let result = deserializer.into_deserializer();
    assert_eq!(result, deserializer);
}

#[test]
fn test_into_deserializer_clone() {
    #[derive(Clone, Debug, PartialEq)]
    struct CloneableDeserializer;

    impl<'de> IntoDeserializer<'de> for CloneableDeserializer {
        type Deserializer = Self;
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let deserializer = CloneableDeserializer;
    let result = deserializer.clone().into_deserializer();
    assert_eq!(result, deserializer);
}

