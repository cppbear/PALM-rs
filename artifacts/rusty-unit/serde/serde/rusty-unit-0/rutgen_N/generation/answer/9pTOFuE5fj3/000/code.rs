// Answer 0

#[test]
fn test_variant_seed_success() {
    use serde::de::{self, DeserializeSeed};

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = i32;

        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            Ok(42)
        }
    }

    struct TestVariant {
        value: i32,
    }

    impl TestVariant {
        fn variant_seed<T>(self, seed: T) -> Result<(T::Value, Self::Variant), String>
        where
            T: DeserializeSeed<'de>,
        {
            seed.deserialize(self).map(|value| (value, self))
                .map_err(|_err| String::from("Error deserializing"))
        }
    }

    let variant = TestVariant { value: 0 };
    let seed = TestSeed;

    let result = variant.variant_seed(seed);
    assert_eq!(result.unwrap().0, 42);
}

#[test]
#[should_panic]
fn test_variant_seed_failure() {
    use serde::de::{self, DeserializeSeed};

    struct FailingSeed;

    impl<'de> DeserializeSeed<'de> for FailingSeed {
        type Value = i32;

        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            Err(de::Error::custom("failed to deserialize"))
        }
    }

    struct TestVariant {
        value: i32,
    }

    impl TestVariant {
        fn variant_seed<T>(self, seed: T) -> Result<(T::Value, Self::Variant), String>
        where
            T: DeserializeSeed<'de>,
        {
            seed.deserialize(self).map(|value| (value, self))
                .map_err(|_err| String::from("Error deserializing"))
        }
    }

    let variant = TestVariant { value: 0 };
    let seed = FailingSeed;

    let _result = variant.variant_seed(seed).unwrap(); // This should panic
}

