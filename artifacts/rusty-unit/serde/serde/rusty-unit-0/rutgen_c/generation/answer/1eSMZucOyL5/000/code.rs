// Answer 0

#[test]
fn test_deserialize_ignored_any_success() {
    struct TestDeserializer;

    impl<'de> Deserializer<'de> for TestDeserializer {
        type Error = crate::de::Error;

        fn deserialize_ignored_any(self, _: IgnoredAny) -> Result<IgnoredAny, Self::Error> {
            Ok(IgnoredAny)
        }
    }

    let deserializer = TestDeserializer;
    let result: Result<IgnoredAny, _> = IgnoredAny::deserialize(deserializer);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_deserialize_ignored_any_fail() {
    struct FailingDeserializer;

    impl<'de> Deserializer<'de> for FailingDeserializer {
        type Error = crate::de::Error;

        fn deserialize_ignored_any(self, _: IgnoredAny) -> Result<IgnoredAny, Self::Error> {
            Err(crate::de::Error::custom("Deserialization failed"))
        }
    }

    let deserializer = FailingDeserializer;
    let result: Result<IgnoredAny, _> = IgnoredAny::deserialize(deserializer);
    assert!(result.is_err());
}

