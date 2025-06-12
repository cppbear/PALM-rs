// Answer 0

#[test]
fn test_or_insert_occupied_entry() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;
    use std::hash::Hash;
    use indexmap::map::Entry;

    struct TestEntry<K, V> {
        key: K,
        value: V,
    }

    impl<K, V> Entry<K, V> {
        fn into_key_value_mut(&mut self) -> (&mut K, &mut V) {
            (&mut self.key, &mut self.value)
        }
    }

    struct TestMap<K, V, S: BuildHasher> {
        entry: Option<TestEntry<K, V>>,
        hasher: S,
    }

    impl<K: Hash + Eq, V, S: BuildHasher> TestMap<K, V, S> {
        fn or_insert(self, default_key: K, default_value: V) -> (&mut K, &mut V) {
            match self.entry {
                Some(ref mut entry) => entry.into_key_value_mut(),
                None => {
                    self.entry = Some(TestEntry {
                        key: default_key,
                        value: default_value,
                    });
                    (&mut self.entry.as_mut().unwrap().key, &mut self.entry.as_mut().unwrap().value)
                },
            }
        }
    }

    let default_key = String::from("key1");
    let default_value = String::from("value1");
    
    // Create an occupied entry
    let mut test_map = TestMap {
        entry: Some(TestEntry {
            key: String::from("existing_key"),
            value: String::from("existing_value"),
        }),
        hasher: DefaultHasher::new(),
    };

    // Call or_insert and expect it to return the existing key and value
    let (key_ref, value_ref) = test_map.or_insert(default_key.clone(), default_value.clone());

    assert_eq!(key_ref, &String::from("existing_key"));
    assert_eq!(value_ref, &String::from("existing_value"));
}

