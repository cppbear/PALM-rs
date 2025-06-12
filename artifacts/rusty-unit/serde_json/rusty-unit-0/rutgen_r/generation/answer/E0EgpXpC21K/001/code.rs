// Answer 0

#[test]
fn test_variant_seed_success() {
    use serde::de::{DeserializeSeed, Visitor};
    use serde::{Deserialize, Serialize};
    use serde_json::Error;
    use std::marker::PhantomData;

    #[derive(Debug, Deserialize, Serialize)]
    enum TestVariant {
        VariantA,
        VariantB,
    }

    struct TestSeed {
        expected_value: i32,
    }

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = i32;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_i32(TestVisitor { expected_value: self.expected_value })
        }
    }

    struct TestVisitor {
        expected_value: i32,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i32 value")
        }

        fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if value == self.expected_value {
                Ok(value)
            } else {
                Err(E::custom("value does not match expected"))
            }
        }
    }

    let variant_value = TestVariant::VariantB;
    let expected_value = 42;
    let seed = TestSeed { expected_value };

    let result: Result<(i32, _), Error> = variant_seed(variant_value, seed);
    
    assert!(result.is_ok());
    let (value, _visitor) = result.unwrap();
    assert_eq!(value, expected_value);
}

#[test]
fn test_variant_seed_failure() {
    use serde::de::{DeserializeSeed, Visitor};
    use serde::{Deserialize, Serialize};
    use serde_json::Error;

    #[derive(Debug, Deserialize, Serialize)]
    enum TestVariant {
        VariantA,
        VariantB,
    }

    struct TestSeed {
        expected_value: i32,
    }

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = i32;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_i32(TestVisitor { expected_value: self.expected_value })
        }
    }

    struct TestVisitor {
        expected_value: i32,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i32 value")
        }

        fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if value == self.expected_value {
                Ok(value)
            } else {
                Err(E::custom("value does not match expected"))
            }
        }
    }

    let variant_value = TestVariant::VariantA;
    let invalid_value = 99;
    let seed = TestSeed { expected_value: 42 };

    let result: Result<(i32, _), Error> = variant_seed(variant_value, seed);
    
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "value does not match expected");
}

