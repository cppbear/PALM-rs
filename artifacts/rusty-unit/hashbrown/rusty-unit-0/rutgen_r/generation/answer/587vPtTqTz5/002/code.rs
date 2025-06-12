// Answer 0

#[test]
fn test_get_inner_non_empty_table() {
    use hashbrown::HashMap;
    use std::hash::{BuildHasher, Hash};

    struct CustomHasher;
    impl BuildHasher for CustomHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    struct KeyWrapper<'a>(&'a str);
    
    impl Hash for KeyWrapper<'_> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    struct MyHashMap<K, V> {
        table: HashMap<K, V, CustomHasher>,
        hash_builder: CustomHasher,
    }

    impl<K, V> MyHashMap<K, V> {
        fn new() -> Self {
            Self {
                table: HashMap::new(),
                hash_builder: CustomHasher,
            }
        }

        fn get_inner<Q>(&self, k: &Q) -> Option<&(K, V)>
        where
            Q: Hash + Equivalent<K> + ?Sized,
        {
            if self.table.is_empty() {
                None
            } else {
                let hash = make_hash::<Q, CustomHasher>(&self.hash_builder, k);
                self.table.get(hash, equivalent_key(k))
            }
        }

        fn insert(&mut self, key: K, value: V) {
            self.table.insert(key, value);
        }
    }

    // Dummy functions for missing parts
    fn make_hash<Q, S>(_hasher: &S, _key: &Q) -> u64 {
        0 // Placeholder logic
    }

    fn equivalent_key<K, Q>(_key: &Q) -> &K {
        unimplemented!() // Placeholder logic
    }

    // Set up test to ensure table is not empty
    let mut my_map = MyHashMap::new();
    let key = "test_key";
    let value = 42;
    my_map.insert(key.to_string(), value);

    let result = my_map.get_inner(&KeyWrapper(key));

    assert!(result.is_some());
    let (stored_key, stored_value) = result.unwrap();
    assert_eq!(stored_key, &key.to_string());
    assert_eq!(stored_value, &value);
}

#[test]
#[should_panic]
fn test_get_inner_empty_table() {
    let my_map: MyHashMap<String, i32> = MyHashMap::new();
    let key = "key_not_inserted";
    let _ = my_map.get_inner(&KeyWrapper(key)); // Should panic since the table is empty
}

