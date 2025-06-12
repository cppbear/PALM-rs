// Answer 0

#[test]
fn test_next_entry_with_valid_key_value() {
    struct ValidKeyValue;

    impl serde::de::Deserialize<'_> for ValidKeyValue {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'_>,
        {
            Ok(ValidKeyValue)
        }
    }

    struct TestMap {
        data: Vec<(ValidKeyValue, ValidKeyValue)>,
        index: usize,
    }

    impl<'de> serde::de::MapAccess<'de> for TestMap {
        type Error = std::convert::Infallible;

        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: serde::de::DeserializeSeed<'de>,
        {
            if self.index < self.data.len() {
                self.index += 1; // Move to the next key
                seed.deserialize(serde::de::value::BorrowedNoneDeserializer)
            } else {
                Ok(None)
            }
        }

        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::DeserializeSeed<'de>,
        {
            if self.index <= self.data.len() {
                seed.deserialize(serde::de::value::BorrowedNoneDeserializer)
            } else {
                Err(std::convert::Infallible)
            }
        }
    }

    let mut map_access = TestMap {
        data: vec![(ValidKeyValue, ValidKeyValue)],
        index: 0,
    };

    let entry: Option<(ValidKeyValue, ValidKeyValue)> = map_access.next_entry().unwrap();
    assert!(entry.is_some());
}

#[test]
fn test_next_entry_with_no_items() {
    struct ValidKeyValue;

    impl serde::de::Deserialize<'_> for ValidKeyValue {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'_>,
        {
            Ok(ValidKeyValue)
        }
    }

    struct EmptyMap;

    impl<'de> serde::de::MapAccess<'de> for EmptyMap {
        type Error = std::convert::Infallible;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: serde::de::DeserializeSeed<'de>,
        {
            Ok(None)
        }

        fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::DeserializeSeed<'de>,
        {
            Err(std::convert::Infallible)
        }
    }

    let mut empty_map = EmptyMap;

    let entry: Option<(ValidKeyValue, ValidKeyValue)> = empty_map.next_entry().unwrap();
    assert!(entry.is_none());
}

#[test]
#[should_panic]
fn test_next_entry_panic_on_invalid_data() {
    struct InvalidKey;

    impl serde::de::Deserialize<'_> for InvalidKey {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'_>,
        {
            Err(serde::de::Error::custom("Invalid Key"))
        }
    }

    struct InvalidMap;

    impl<'de> serde::de::MapAccess<'de> for InvalidMap {
        type Error = std::convert::Infallible;

        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: serde::de::DeserializeSeed<'de>,
        {
            seed.deserialize(serde::de::value::BorrowedNoneDeserializer)
        }

        fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::DeserializeSeed<'de>,
        {
            Err(std::convert::Infallible)
        }
    }

    let mut invalid_map = InvalidMap;
    let _entry: Option<(InvalidKey, ValidKey)> = invalid_map.next_entry().unwrap();
}

