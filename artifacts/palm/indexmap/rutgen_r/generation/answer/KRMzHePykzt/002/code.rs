// Answer 0

#[test]
fn test_get_found_value() {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    struct Key(String);
    
    impl PartialEq for Key {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for Key {}

    impl Hash for Key {
        fn hash<H: Hasher>(&self, state: &mut H) {
            state.write(self.0.as_bytes());
        }
    }

    struct Value(i32);

    struct Map {
        entries: Vec<(Key, Value)>,
    }

    impl Map {
        fn new() -> Self {
            Map { entries: Vec::new() }
        }

        fn insert(&mut self, key: Key, value: Value) {
            self.entries.push((key, value));
        }

        fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
        where
            Q: ?Sized + Hash + Equivalent<Key>,
        {
            self.entries.iter().position(|(k, _)| k == key)
        }

        fn as_entries(&self) -> &[(Key, Value)] {
            &self.entries
        }

        pub fn get<Q>(&self, key: &Q) -> Option<&Value>
        where
            Q: ?Sized + Hash + Equivalent<Key>,
        {
            if let Some(i) = self.get_index_of(key) {
                let entry = &self.as_entries()[i];
                Some(&entry.1)
            } else {
                None
            }
        }
    }

    impl Equivalent<Key> for Key {
        fn equivalent(&self, other: &Key) -> bool {
            self.eq(other)
        }
    }

    let mut map = Map::new();
    let key = Key("test".to_string());
    let value = Value(42);

    map.insert(key.clone(), value);

    let result = map.get(&key);
    assert_eq!(result, Some(&Value(42)));
}

