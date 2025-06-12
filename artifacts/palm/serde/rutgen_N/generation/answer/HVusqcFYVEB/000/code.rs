// Answer 0

#[test]
fn test_variant_seed_success() {
    use serde::de::{self, Deserialize, DeserializeSeed, Deserializer};
    use serde::ser::Serializer;

    struct TestVariant;
    
    impl de::VariantAccess for TestVariant {
        type Error = serde::de::value::Error;

        fn unit_variant(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = usize;

        fn deserialize<D>(self, deserializer: D) -> Result<usize, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_u64(42u64)
        }
    }

    let result: Result<(usize, TestVariant), TestVariant::Error> = TestSeed.deserialize(TestVariant);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().0, 42);
}

#[test]
#[should_panic]
fn test_variant_seed_failure() {
    use serde::de::{self, DeserializeSeed, Deserializer};

    struct PanicVariant;

    impl de::VariantAccess for PanicVariant {
        type Error = serde::de::value::Error;

        fn unit_variant(self) -> Result<(), Self::Error> {
            panic!("this is a panic");
        }
    }

    struct PanicSeed;

    impl<'de> DeserializeSeed<'de> for PanicSeed {
        type Value = usize;

        fn deserialize<D>(self, deserializer: D) -> Result<usize, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_u64(43u64)
        }
    }

    let _: Result<(usize, PanicVariant), PanicVariant::Error> = PanicSeed.deserialize(PanicVariant);
}

