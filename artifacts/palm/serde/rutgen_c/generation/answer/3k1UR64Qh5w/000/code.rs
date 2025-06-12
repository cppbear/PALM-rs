// Answer 0

#[test]
fn test_variant_seed_success() {
    struct MockMapAccess {
        keys: Vec<u8>,
        index: usize,
    }

    impl<'de> de::MapAccess<'de> for MockMapAccess {
        type Error = ();

        fn next_key_seed<T>(&mut self, _: T) -> Result<Option<u8>, Self::Error>
        where
            T: de::DeserializeSeed<'de>,
        {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }
    }

    let mock_map = MockMapAccess {
        keys: vec![1, 2, 3],
        index: 0,
    };
    let map_access_deserializer = MapAccessDeserializer { map: mock_map };
    let result = map_access_deserializer.variant_seed(crate::de::DummyDeserializeSeed);

    assert!(result.is_ok());
    let (key, _) = result.unwrap();
    assert_eq!(key, 1);
}

#[test]
fn test_variant_seed_failure() {
    struct EmptyMapAccess;

    impl<'de> de::MapAccess<'de> for EmptyMapAccess {
        type Error = ();

        fn next_key_seed<T>(&mut self, _: T) -> Result<Option<u8>, Self::Error>
        where
            T: de::DeserializeSeed<'de>,
        {
            Ok(None)
        }
    }

    let empty_map = EmptyMapAccess;
    let map_access_deserializer = MapAccessDeserializer { map: empty_map };
    let result = map_access_deserializer.variant_seed(crate::de::DummyDeserializeSeed);

    assert!(result.is_err());
}

