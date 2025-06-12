// Answer 0

#[test]
fn test_deserialize_unit_success() {
    struct DummyDeserializer;

    impl<'de> Deserializer<'de> for DummyDeserializer {
        type Error = serde::de::value::Error;

        fn deserialize_unit<D>(self, _visitor: D) -> Result<Self::Ok, Self::Error>
        where
            D: serde::de::Visitor<'de>,
        {
            Ok(())
        }

        fn is_human_readable(&self) -> bool {
            true
        }
    }

    let deserializer = DummyDeserializer;
    let result: Result<(), _> = deserialize(deserializer);
    assert!(result.is_ok());
}

#[should_panic(expected = "invalid value")]
#[test]
fn test_deserialize_unit_failure() {
    struct FailingDeserializer;

    impl<'de> Deserializer<'de> for FailingDeserializer {
        type Error = serde::de::value::Error;

        fn deserialize_unit<D>(self, _visitor: D) -> Result<Self::Ok, Self::Error>
        where
            D: serde::de::Visitor<'de>,
        {
            Err(serde::de::value::Error::custom("invalid value"))
        }

        fn is_human_readable(&self) -> bool {
            true
        }
    }

    let deserializer = FailingDeserializer;
    let _result: Result<(), _> = deserialize(deserializer);
}

