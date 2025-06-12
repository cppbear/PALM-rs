// Answer 0

#[test]
fn test_next_key_seed_with_valid_key() {
    struct DummySeed {
        key: String,
    }

    impl<'de> DeserializeSeed<'de> for DummySeed {
        type Value = String;

        fn deserialize<DS>(self, _: DS) -> Result<Self::Value, Error>
        where
            DS: Deserializer<'de>,
        {
            Ok(self.key)
        }
    }

    let mut map = Map {
        map: MapImpl::new(),
    };
    map.insert("key1".to_string(), Value::Bool(true));

    let iter = map.map.iter_mut();

    let mut deserializer = MapRefDeserializer {
        iter,
        value: None,
    };
    
    let seed = DummySeed {
        key: "key1".to_string(),
    };

    let result = deserializer.next_key_seed(seed);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), "key1");
}

#[test]
fn test_next_key_seed_no_keys() {
    struct DummySeed {
        key: String,
    }

    impl<'de> DeserializeSeed<'de> for DummySeed {
        type Value = String;

        fn deserialize<DS>(self, _: DS) -> Result<Self::Value, Error>
        where
            DS: Deserializer<'de>,
        {
            Ok(self.key)
        }
    }

    let map = Map {
        map: MapImpl::new(),
    };

    let iter = map.map.iter_mut();

    let mut deserializer = MapRefDeserializer {
        iter,
        value: None,
    };

    let seed = DummySeed {
        key: "nonexistent".to_string(),
    };

    let result = deserializer.next_key_seed(seed);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
}

