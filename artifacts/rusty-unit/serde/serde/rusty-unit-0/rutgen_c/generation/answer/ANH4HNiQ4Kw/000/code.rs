// Answer 0

#[test]
fn test_next_key_seed_with_some_key() {
    use crate::de::value::Content;
    use crate::de::DeserializeSeed;

    struct TestSeed;
    
    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<T>(self, _deserializer: T) -> Result<Self::Value, T::Error> 
        where
            T: Deserializer<'de>,
        {
            Ok("Test".to_owned())
        }
    }

    let content_1 = Content::String("Key1".to_string());
    let content_2 = Content::String("Value1".to_string());
    let pair_1 = Some((content_1.clone(), content_2.clone()));
    let pair_2 = Some((Content::String("Key2".to_string()), Content::String("Value2".to_string())));
    let pairs = vec![pair_1, pair_2];
    
    let mut flat_map_access = FlatMapAccess {
        iter: pairs.iter(),
        pending_content: None,
        _marker: std::marker::PhantomData,
    };

    let result = flat_map_access.next_key_seed(TestSeed);

    assert_eq!(result.unwrap(), Some("Key1".to_string()));
}

#[test]
fn test_next_key_seed_with_none_key() {
    use crate::de::value::Content;
    use crate::de::DeserializeSeed;

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<T>(self, _deserializer: T) -> Result<Self::Value, T::Error> 
        where
            T: Deserializer<'de>,
        {
            Ok("Test".to_owned())
        }
    }

    let pairs: Vec<Option<(Content, Content)>> = vec![None, None];
    
    let mut flat_map_access = FlatMapAccess {
        iter: pairs.iter(),
        pending_content: None,
        _marker: std::marker::PhantomData,
    };

    let result = flat_map_access.next_key_seed(TestSeed);

    assert_eq!(result.unwrap(), None);
}

#[test]
fn test_next_value_seed_with_pending_content() {
    use crate::de::value::Content;
    use crate::de::DeserializeSeed;

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<T>(self, _deserializer: T) -> Result<Self::Value, T::Error> 
        where
            T: Deserializer<'de>,
        {
            Ok("Test".to_owned())
        }
    }

    let content_key = Content::String("Key".to_string());
    let content_value = Content::String("Value".to_string());
    let pair = Some((content_key.clone(), content_value.clone()));
    let pairs = vec![pair];

    let mut flat_map_access = FlatMapAccess {
        iter: pairs.iter(),
        pending_content: None,
        _marker: std::marker::PhantomData,
    };

    let _ = flat_map_access.next_key_seed(TestSeed).unwrap();
    let value_result = flat_map_access.next_value_seed(TestSeed);
    
    assert_eq!(value_result.unwrap(), "Value".to_string());
}

#[test]
fn test_next_value_seed_without_pending_content() {
    use crate::de::value::Content;
    use crate::de::DeserializeSeed;

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<T>(self, _deserializer: T) -> Result<Self::Value, T::Error> 
        where
            T: Deserializer<'de>,
        {
            Ok("Test".to_owned())
        }
    }

    let pairs: Vec<Option<(Content, Content)>> = Vec::new();
    
    let mut flat_map_access = FlatMapAccess {
        iter: pairs.iter(),
        pending_content: None,
        _marker: std::marker::PhantomData,
    };

    let value_result = flat_map_access.next_value_seed(TestSeed);
    
    assert!(value_result.is_err());
}

