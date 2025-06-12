// Answer 0

#[test]
fn test_next_key_seed_empty() {
    struct TestSeed;

    impl<'de> de::DeserializeSeed<'de> for TestSeed {
        type Value = ();

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_unit(self)
        }
    }

    struct TestMapAccess {
        iter: std::iter::Fuse<std::iter::Empty<(i32, i32)>>,
    }

    impl<'de> de::MapAccess<'de> for TestMapAccess {
        type Error = ();

        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: de::DeserializeSeed<'de>,
        {
            match self.next_pair() {
                Some(_) => Ok(Some(seed.deserialize(&mut self.iter)?)),
                None => Ok(None),
            }
        }

        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: de::DeserializeSeed<'de>,
        {
            Err(())
        }

        fn next_entry_seed<K, V>(&mut self, _kseed: K, _vseed: V) -> Result<Option<(K::Value, V::Value)>, Self::Error>
        where
            K: de::DeserializeSeed<'de>,
            V: de::DeserializeSeed<'de>,
        {
            Err(())
        }

        fn size_hint(&self) -> Option<usize> {
            Some(0)
        }

        fn next_pair(&mut self) -> Option<(i32, i32)> {
            None
        }
    }

    let mut deserializer = TestMapAccess {
        iter: std::iter::empty().fuse(),
    };
    let seed = TestSeed;

    let result = deserializer.next_key_seed(seed);
}

