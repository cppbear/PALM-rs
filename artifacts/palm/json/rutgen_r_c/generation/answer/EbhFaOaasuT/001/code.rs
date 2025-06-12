// Answer 0

fn test_next_key_seed_some() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Debug)]
    struct TestSeed {
        key: Option<String>,
    }

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<DES>(self, deserializer: DES) -> Result<Self::Value, Error>
        where
            DES: Deserializer<'de>,
        {
            let key_de: MapKeyDeserializer<'de> = deserializer
                .deserialize_any(MapKeyDeserializer { key: Cow::Owned("test_key".into()) })?;
            Ok(key_de.key.into_owned())
        }
    }

    let mut map = Map::new();
    map.insert("test_key".to_string(), Value::Bool(true));
    
    let mut deserializer = MapDeserializer {
        iter: map.into_iter(),
        value: None,
    };

    let seed = TestSeed { key: None };

    let result = deserializer.next_key_seed(seed)?;
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "test_key");

    Ok(())
}

fn test_next_key_seed_none() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Debug)]
    struct TestSeed {
        key: Option<String>,
    }

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<DES>(self, deserializer: DES) -> Result<Self::Value, Error>
        where
            DES: Deserializer<'de>,
        {
            let key_de: MapKeyDeserializer<'de> = deserializer
                .deserialize_any(MapKeyDeserializer { key: Cow::Owned("unused".into()) })?;
            Ok(key_de.key.into_owned())
        }
    }

    let mut deserializer = MapDeserializer {
        iter: std::iter::empty::<(String, Value)>().into_iter(),
        value: None,
    };

    let seed = TestSeed { key: None };

    let result = deserializer.next_key_seed(seed)?;
    assert!(result.is_none());

    Ok(())
}

#[test]
fn run_tests() {
    test_next_key_seed_some().unwrap();
    test_next_key_seed_none().unwrap();
}

