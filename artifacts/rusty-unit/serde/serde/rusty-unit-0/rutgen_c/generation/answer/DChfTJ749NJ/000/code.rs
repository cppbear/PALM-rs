// Answer 0

#[test]
fn test_variant_seed_success() {
    use crate::de::{Deserializer, DeserializeSeed};
    
    struct TestSeed;
    
    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(&mut Visitor)
        }
    }

    struct Visitor;

    impl<'de> crate::de::Visitor<'de> for Visitor {
        type Value = String;

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }
        
        // Implement other `visit_*` methods as no-ops
        fn visit_unit<E>(self) -> Result<Self::Value, E> where E: de::Error {
            Ok(String::new())
        }
    }

    let string_deserializer = StringDeserializer {
        value: "test".to_string(),
        marker: PhantomData,
    };

    let result: Result<(String, UnitOnly<()>), _> = string_deserializer.variant_seed(TestSeed);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().0, "test");
}

#[test]
#[should_panic]
fn test_variant_seed_failure() {
    use crate::de::{Deserializer, DeserializeSeed};

    struct BadSeed;

    impl<'de> DeserializeSeed<'de> for BadSeed {
        type Value = String;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Err(D::Error::custom("error"))
        }
    }

    let string_deserializer = StringDeserializer {
        value: "test".to_string(),
        marker: PhantomData,
    };

    let _result: Result<(String, UnitOnly<()>), _> = string_deserializer.variant_seed(BadSeed);
}

