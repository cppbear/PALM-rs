// Answer 0

#[test]
fn test_variant_seed_success() {
    use serde::de::{DeserializeSeed, Deserializer};
    use serde_json::Value;

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = i32;

        fn deserialize<D>(&self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            let value = i32::deserialize(deserializer)?;
            Ok(value)
        }
    }

    struct TestVariant {
        variant: &'static str,
        value: Value,
    }

    impl TestVariant {
        fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self), serde_json::Error>
        where
            V: DeserializeSeed<'de>,
        {
            let variant = self.variant;
            let visitor = VariantRefDeserializer { value: self.value };
            seed.deserialize(variant).map(|v| (v, visitor))
        }
    }

    struct VariantRefDeserializer {
        value: Value,
    }

    let test_variant = TestVariant {
        variant: "42",
        value: Value::Null,
    };

    let result = test_variant.variant_seed(TestSeed);

    assert!(result.is_ok());
    if let Ok((value, _)) = result {
        assert_eq!(value, 42);
    }
}

#[test]
#[should_panic]
fn test_variant_seed_failure() {
    use serde::de::{DeserializeSeed, Deserializer};
    use serde_json::Value;

    struct FailSeed;

    impl<'de> DeserializeSeed<'de> for FailSeed {
        type Value = i32;

        fn deserialize<D>(&self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Err(serde::de::Error::custom("deserialization error"))
        }
    }

    struct TestVariant {
        variant: &'static str,
        value: Value,
    }

    impl TestVariant {
        fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self), serde_json::Error>
        where
            V: DeserializeSeed<'de>,
        {
            let variant = self.variant;
            let visitor = VariantRefDeserializer { value: self.value };
            seed.deserialize(variant).map(|v| (v, visitor))
        }
    }

    struct VariantRefDeserializer {
        value: Value,
    }

    let test_variant = TestVariant {
        variant: "invalid",
        value: Value::Null,
    };

    let _result = test_variant.variant_seed(FailSeed);
}

