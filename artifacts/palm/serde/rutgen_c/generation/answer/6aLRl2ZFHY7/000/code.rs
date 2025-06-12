// Answer 0

#[test]
fn test_next_value_seed_with_some_value() {
    struct DummySeed;

    impl<'de> DeserializeSeed<'de> for DummySeed {
        type Value = String;

        fn deserialize<T>(self, _: T) -> Result<Self::Value, T::Error>
        where
            T: Deserializer<'de>,
        {
            Ok("dummy value".to_string())
        }
    }

    let content = Content::String("test".to_string());
    let mut access = FlatStructAccess {
        iter: &mut [].iter_mut(),
        pending_content: Some(content),
        fields: &["field1"],
        _marker: PhantomData,
    };

    let result: Result<String, _> = access.next_value_seed(DummySeed);
    assert_eq!(result.unwrap(), "dummy value");
}

#[test]
fn test_next_value_seed_with_none_value() {
    struct DummySeed;

    impl<'de> DeserializeSeed<'de> for DummySeed {
        type Value = String;

        fn deserialize<T>(self, _: T) -> Result<Self::Value, T::Error>
        where
            T: Deserializer<'de>,
        {
            Ok("dummy value".to_string())
        }
    }

    let mut access = FlatStructAccess {
        iter: &mut [].iter_mut(),
        pending_content: None,
        fields: &["field1"],
        _marker: PhantomData,
    };

    let result: Result<String, _> = access.next_value_seed(DummySeed);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "value is missing");
}

