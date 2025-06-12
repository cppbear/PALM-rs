// Answer 0

#[test]
fn test_newtype_variant_seed_valid() {
    struct ValidSeed;
    
    impl<'de> de::DeserializeSeed<'de> for ValidSeed {
        type Value = u32;
        fn deserialize<S>(self, _deserializer: S) -> Result<Self::Value, de::Error>
        where
            S: Deserializer<'de>,
        {
            Ok(42)
        }
    }

    let access = UnitOnly {};
    let result = access.newtype_variant_seed(ValidSeed {});
}

#[test]
fn test_newtype_variant_seed_invalid() {
    struct InvalidSeed;

    impl<'de> de::DeserializeSeed<'de> for InvalidSeed {
        type Value = String;
        fn deserialize<S>(self, _deserializer: S) -> Result<Self::Value, de::Error>
        where
            S: Deserializer<'de>,
        {
            Err(de::Error::custom("error"))
        }
    }

    let access = UnitOnly {};
    let result = access.newtype_variant_seed(InvalidSeed {});
}

#[test]
#[should_panic]
fn test_newtype_variant_seed_panic() {
    struct PanicSeed;

    impl<'de> de::DeserializeSeed<'de> for PanicSeed {
        type Value = ();
        fn deserialize<S>(self, _deserializer: S) -> Result<Self::Value, de::Error>
        where
            S: Deserializer<'de>,
        {
            panic!("This seed always panics");
        }
    }

    let access = UnitOnly {};
    let _result = access.newtype_variant_seed(PanicSeed {});
}

