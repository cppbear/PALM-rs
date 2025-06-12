// Answer 0

#[test]
fn test_insert_full_new_entry() {
    struct TestMap<K, V> {
        core: std::collections::HashMap<K, V>,
    }

    impl<K: std::hash::Hash + Eq, V> TestMap<K, V> {
        fn hash(&self, key: &K) -> u64 {
            // Simple hash function for demonstration.
            let hasher = std::collections::hash_map::DefaultHasher::new();
            hasher.write(&bincode::serialize(key).unwrap());
            hasher.finish()
        }

        fn insert_full(&mut self, key: K, value: V) -> (usize, Option<V>) {
            let index = self.core.len();
            let old_value = self.core.insert(key, value);
            (index, old_value)
        }
    }

    let mut map = TestMap { core: std::collections::HashMap::new() };
    let (index, old_value) = map.insert_full("key1", "value1");
    
    assert_eq!(index, 0);
    assert_eq!(old_value, None);
}

#[test]
fn test_insert_full_update_entry() {
    struct TestMap<K, V> {
        core: std::collections::HashMap<K, V>,
    }

    impl<K: std::hash::Hash + Eq, V> TestMap<K, V> {
        fn hash(&self, key: &K) -> u64 {
            let hasher = std::collections::hash_map::DefaultHasher::new();
            hasher.write(&bincode::serialize(key).unwrap());
            hasher.finish()
        }

        fn insert_full(&mut self, key: K, value: V) -> (usize, Option<V>) {
            let index = self.core.len();
            let old_value = self.core.insert(key, value);
            (index, old_value)
        }
    }

    let mut map = TestMap { core: std::collections::HashMap::new() };
    map.insert_full("key1", "value1");
    let (index, old_value) = map.insert_full("key1", "value2");

    assert_eq!(index, 1);
    assert_eq!(old_value, Some("value1"));
}

#[test]
fn test_insert_full_edge_case_empty() {
    struct TestMap<K, V> {
        core: std::collections::HashMap<K, V>,
    }

    impl<K: std::hash::Hash + Eq, V> TestMap<K, V> {
        fn hash(&self, key: &K) -> u64 {
            let hasher = std::collections::hash_map::DefaultHasher::new();
            hasher.write(&bincode::serialize(key).unwrap());
            hasher.finish()
        }

        fn insert_full(&mut self, key: K, value: V) -> (usize, Option<V>) {
            let index = self.core.len();
            let old_value = self.core.insert(key, value);
            (index, old_value)
        }
    }

    let mut map: TestMap<String, String> = TestMap { core: std::collections::HashMap::new() };
    let (index, old_value) = map.insert_full("key1".to_string(), "value1".to_string());

    assert_eq!(index, 0);
    assert_eq!(old_value, None);
}

#[test]
#[should_panic]
fn test_insert_full_panic_on_hash() {
    struct TestMap<K, V> {
        core: std::collections::HashMap<K, V>,
    }

    impl<K: std::hash::Hash + Eq, V> TestMap<K, V> {
        fn hash(&self, _: &K) -> u64 {
            panic!("Intentional panic for testing");
        }

        fn insert_full(&mut self, key: K, value: V) -> (usize, Option<V>) {
            let index = self.core.len();
            let old_value = self.core.insert(key, value);
            (index, old_value)
        }
    }

    let mut map = TestMap { core: std::collections::HashMap::new() };
    let _ = map.insert_full("key1", "value1");
}

