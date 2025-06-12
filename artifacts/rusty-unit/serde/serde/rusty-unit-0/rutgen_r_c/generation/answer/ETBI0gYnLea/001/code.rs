// Answer 0

#[test]
fn test_variant_seed_with_valid_seed() {
    use crate::de::{self, DeserializeSeed, Visitor};

    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = &'de str;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid string")
        }

        fn visit_str<E>(self, value: &'de str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
    }

    struct MockSeed;

    impl<'de> DeserializeSeed<'de> for MockSeed {
        type Value = &'de str;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_str(MockVisitor)
        }
    }

    let deserializer = StrDeserializer::new("test");
    let result: Result<(&str, UnitOnly<()>, &str), _> = deserializer.variant_seed(MockSeed);
    assert!(result.is_ok());
    let (value, _variant) = result.unwrap();
    assert_eq!(value, "test");
}

#[test]
#[should_panic]
fn test_variant_seed_with_invalid_seed() {
    use crate::de::{self, DeserializeSeed, Visitor};

    struct PanicVisitor;

    impl<'de> Visitor<'de> for PanicVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("any value")
        }

        fn visit_str<E>(self, _value: &'de str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            panic!("This visitor panics");
        }
    }

    struct PanicSeed;

    impl<'de> DeserializeSeed<'de> for PanicSeed {
        type Value = ();

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_str(PanicVisitor)
        }
    }

    let deserializer = StrDeserializer::new("panic_test");
    let _result: Result<(&str, UnitOnly<()>, ()), _> = deserializer.variant_seed(PanicSeed);
}

