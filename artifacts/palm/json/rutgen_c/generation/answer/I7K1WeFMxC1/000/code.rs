// Answer 0

#[test]
fn test_newtype_variant_seed_with_some_value() {
    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = bool;
        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, Error>
        where
            D: Deserializer<'de>,
        {
            Ok(true)
        }
    }

    let variant = VariantDeserializer {
        value: Some(Value::Bool(true)),
    };

    let result: Result<bool, Error> = variant.newtype_variant_seed(TestSeed);
    assert_eq!(result, Ok(true));
}

#[test]
fn test_newtype_variant_seed_with_none_value() {
    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = bool;
        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, Error>
        where
            D: Deserializer<'de>,
        {
            Ok(true)
        }
    }

    let variant = VariantDeserializer {
        value: None,
    };

    let result: Result<bool, Error> = variant.newtype_variant_seed(TestSeed);
    assert!(result.is_err());
}

