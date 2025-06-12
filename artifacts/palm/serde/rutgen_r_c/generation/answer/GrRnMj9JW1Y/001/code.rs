// Answer 0

#[test]
fn test_size_hint_none() {
    struct TestMapAccess;

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = std::fmt::Error;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            Ok(None)
        }

        fn next_value_seed<V>(&mut self, _: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            Err(std::fmt::Error)
        }
    }

    let map_access = TestMapAccess {};
    assert_eq!(map_access.size_hint(), None);
}

