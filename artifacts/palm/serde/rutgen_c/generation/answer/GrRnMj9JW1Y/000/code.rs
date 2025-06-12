// Answer 0

#[test]
fn test_size_hint_none() {
    struct TestMapAccess;

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = Error;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            unimplemented!()
        }

        fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            unimplemented!()
        }
    }

    let map_access = TestMapAccess;
    assert_eq!(map_access.size_hint(), None);
}

#[test]
fn test_size_hint_some() {
    struct TestMapAccessWithHint {
        hint: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccessWithHint {
        type Error = Error;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            unimplemented!()
        }

        fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            unimplemented!()
        }

        fn size_hint(&self) -> Option<usize> {
            Some(self.hint)
        }
    }

    let mut map_access = TestMapAccessWithHint { hint: 5 };
    assert_eq!(map_access.size_hint(), Some(5));
}

