// Answer 0

#[test]
fn test_newtype_variant_seed_with_some_value() {
    struct TestSeed;

    impl<'de> de::DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            let s: String = Deserialize::deserialize(deserializer)?;
            Ok(s)
        }
    }

    struct TestStruct {
        value: Option<String>,
    }

    impl TestStruct {
        fn new(value: Option<String>) -> Self {
            TestStruct { value }
        }

        fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, serde::de::Error>
        where
            T: serde::de::DeserializeSeed<'de>,
        {
            match self.value {
                Some(value) => seed.deserialize(ContentRefDeserializer::new(value)),
                None => Err(serde::de::Error::invalid_type(
                    serde::de::Unexpected::UnitVariant,
                    &"newtype variant",
                )),
            }
        }
    }

    let test_struct = TestStruct::new(Some("test".to_string()));
    let result = test_struct.newtype_variant_seed(TestSeed);

    assert_eq!(result.unwrap(), "test".to_string());
}

#[test]
#[should_panic]
fn test_newtype_variant_seed_with_none_value() {
    struct TestSeed;

    impl<'de> de::DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            let s: String = Deserialize::deserialize(deserializer)?;
            Ok(s)
        }
    }

    struct TestStruct {
        value: Option<String>,
    }

    impl TestStruct {
        fn new(value: Option<String>) -> Self {
            TestStruct { value }
        }

        fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, serde::de::Error>
        where
            T: serde::de::DeserializeSeed<'de>,
        {
            match self.value {
                Some(value) => seed.deserialize(ContentRefDeserializer::new(value)),
                None => Err(serde::de::Error::invalid_type(
                    serde::de::Unexpected::UnitVariant,
                    &"newtype variant",
                )),
            }
        }
    }

    let test_struct = TestStruct::new(None);
    let _ = test_struct.newtype_variant_seed(TestSeed);
}

