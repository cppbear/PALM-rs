// Answer 0

#[test]
fn test_insert_full_vacant_entry() {
    use std::collections::HashMap;

    struct HashValue {
        value: usize,
    }
    
    impl HashValue {
        fn get(&self) -> usize {
            self.value
        }
    }
    
    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct CustomMap<K, V> {
        entries: Vec<Entry<K, V>>,
        indices: HashMap<usize, usize>,
    }

    impl<K: Eq, V> CustomMap<K, V> {
        pub(crate) fn insert_full(&mut self, hash: HashValue, key: K, value: V) -> (usize, Option<V>) {
            let eq = self.entries.iter().position(|entry| &entry.key == &key);
            let hasher = hash.get();
            match self.indices.entry(hasher) {
                Entry::Occupied(_) => panic!("Should not be occupied"),
                Entry::Vacant(entry) => {
                    let i = self.entries.len();
                    entry.insert(i);
                    self.entries.push(Entry { key, value });
                    (i, None)
                }
            }
        }
    }

    let mut custom_map = CustomMap {
        entries: Vec::new(),
        indices: HashMap::new(),
    };

    let hash_value = HashValue { value: 0 };
    let key = "test_key";
    let value = "test_value";

    let result = custom_map.insert_full(hash_value, key, value);
    assert_eq!(result, (0, None));
}

