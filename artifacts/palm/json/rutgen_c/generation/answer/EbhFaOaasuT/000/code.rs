// Answer 0

#[test]
fn test_next_key_seed_with_valid_key() {
    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            let key: MapKeyDeserializer = deserializer.deserialize_any(MapKeyDeserializer { key: Cow::Owned("test_key".to_string()) })?;
            Ok(key.key.to_owned())
        }
    }

    let mut map = Map::new();
    map.insert("test_key".to_string(), Value::String("test_value".to_string()));
    
    let iter = map.map.iter().into_iter();
    let mut deserializer = MapDeserializer { iter, value: None };
    
    let result = deserializer.next_key_seed(TestSeed);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some("test_key".to_string()));
}

#[test]
fn test_next_key_seed_with_empty_map() {
    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            let key: MapKeyDeserializer = deserializer.deserialize_any(MapKeyDeserializer { key: Cow::Owned("test_key".to_string()) })?;
            Ok(key.key.to_owned())
        }
    }

    let map = Map::new();
    
    let iter = map.map.iter().into_iter();
    let mut deserializer = MapDeserializer { iter, value: None };
    
    let result = deserializer.next_key_seed(TestSeed);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
}

