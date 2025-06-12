// Answer 0

#[test]
fn test_variant_seed_with_bool() {
    struct DummySeed;
    impl<'de> de::DeserializeSeed<'de> for DummySeed {
        type Value = bool;
        fn deserialize<D>(self, deserializer: D) -> Result<bool, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_bool(true)
        }
    }

    let content = Content::Bool(true);
    let deserializer = EnumRefDeserializer::<Error> {
        variant: &content,
        value: None,
        err: PhantomData,
    };

    let result = deserializer.variant_seed(DummySeed);
    assert!(result.is_ok());
    let (value, _visitor) = result.unwrap();
    assert_eq!(value, true);
}

#[test]
fn test_variant_seed_with_string() {
    struct StringSeed;
    impl<'de> de::DeserializeSeed<'de> for StringSeed {
        type Value = String;
        fn deserialize<D>(self, deserializer: D) -> Result<String, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_string("test".to_string())
        }
    }

    let content = Content::String("test".to_string());
    let deserializer = EnumRefDeserializer::<Error> {
        variant: &content,
        value: None,
        err: PhantomData,
    };

    let result = deserializer.variant_seed(StringSeed);
    assert!(result.is_ok());
    let (value, _visitor) = result.unwrap();
    assert_eq!(value, "test".to_string());
}

#[test]
fn test_variant_seed_with_none_variant() {
    struct NoneSeed;
    impl<'de> de::DeserializeSeed<'de> for NoneSeed {
        type Value = Option<()>;
        fn deserialize<D>(self, deserializer: D) -> Result<Option<()>, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_unit().map(|_| None)
        }
    }

    let content = Content::None;
    let deserializer = EnumRefDeserializer::<Error> {
        variant: &content,
        value: None,
        err: PhantomData,
    };

    let result = deserializer.variant_seed(NoneSeed);
    assert!(result.is_ok());
    let (value, _visitor) = result.unwrap();
    assert_eq!(value, None);
}

