// Answer 0

#[test]
fn test_unit_variant() {
    struct MockMapAccess {
        value: Option<usize>,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = Error;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            Ok(None)
        }

        fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            match self.value {
                Some(val) => Ok(val as V::Value),
                None => Err(Error),
            }
        }
    }

    let map_access = MockMapAccess { value: Some(42) };
    let variant_access = MapAsEnum { map: map_access };

    assert!(variant_access.unit_variant().is_ok());
}

#[test]
#[should_panic]
fn test_unit_variant_empty() {
    struct MockMapAccess {
        value: Option<usize>,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = Error;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            Ok(None)
        }

        fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            match self.value {
                Some(val) => Ok(val as V::Value),
                None => Err(Error),
            }
        }
    }

    let map_access = MockMapAccess { value: None };
    let variant_access = MapAsEnum { map: map_access };

    variant_access.unit_variant().unwrap();
}

