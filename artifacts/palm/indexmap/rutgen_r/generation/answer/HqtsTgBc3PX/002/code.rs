// Answer 0

#[test]
fn test_shift_remove_full_success() {
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};

    #[derive(Debug, Clone, Hash, PartialEq, Eq)]
    struct Key(String);

    #[derive(Debug)]
    struct Value(i32);

    #[derive(Debug)]
    struct MyMap {
        entries: Vec<(Key, Value)>,
        indices: HashMap<u64, usize>, // For the sake of emulation
    }

    impl MyMap {
        fn new() -> Self {
            MyMap {
                entries: Vec::new(),
                indices: HashMap::new(),
            }
        }

        fn insert(&mut self, key: Key, value: Value) {
            let hash = self.calculate_hash(&key);
            self.entries.push((key.clone(), value));
            self.indices.insert(hash, self.entries.len() - 1);
        }

        fn calculate_hash<Q: Hash>(&self, key: &Q) -> u64 {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            key.hash(&mut hasher);
            hasher.finish()
        }

        pub(crate) fn shift_remove_full<Q>(&mut self, hash: u64, key: &Q) -> Option<(usize, Key, Value)>
        where
            Q: ?Sized + PartialEq<Key>,
        {
            if let Some(&index) = self.indices.get(&hash) {
                let (key, value) = self.entries.remove(index);
                self.indices.remove(&hash);
                Some((index, key, value))
            } else {
                None
            }
        }
    }

    let mut my_map = MyMap::new();
    let key1 = Key("test1".to_string());
    let value1 = Value(42);
    my_map.insert(key1.clone(), value1);

    let hash = my_map.calculate_hash(&key1);
    if let Some((index, key, value)) = my_map.shift_remove_full(hash, &key1) {
        assert_eq!(index, 0);
        assert_eq!(key, key1);
        assert_eq!(value, Value(42));
    } else {
        panic!("Entry was expected to be removed successfully");
    }
}

#[test]
fn test_shift_remove_full_not_found() {
    #[derive(Debug, Clone, Hash, PartialEq, Eq)]
    struct Key(String);

    #[derive(Debug)]
    struct Value(i32);

    #[derive(Debug)]
    struct MyMap {
        entries: Vec<(Key, Value)>,
        indices: std::collections::HashMap<u64, usize>,
    }

    impl MyMap {
        fn new() -> Self {
            MyMap {
                entries: Vec::new(),
                indices: std::collections::HashMap::new(),
            }
        }

        fn insert(&mut self, key: Key, value: Value) {
            let hash = self.calculate_hash(&key);
            self.entries.push((key.clone(), value));
            self.indices.insert(hash, self.entries.len() - 1);
        }

        fn calculate_hash<Q: Hash>(&self, key: &Q) -> u64 {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            key.hash(&mut hasher);
            hasher.finish()
        }

        pub(crate) fn shift_remove_full<Q>(&mut self, hash: u64, key: &Q) -> Option<(usize, Key, Value)>
        where
            Q: ?Sized + PartialEq<Key>,
        {
            if let Some(&index) = self.indices.get(&hash) {
                let (key, value) = self.entries.remove(index);
                self.indices.remove(&hash);
                Some((index, key, value))
            } else {
                None
            }
        }
    }

    let mut my_map = MyMap::new();
    
    let key1 = Key("test1".to_string());
    let hash = my_map.calculate_hash(&key1);
    let result = my_map.shift_remove_full(hash, &key1);
    
    assert_eq!(result, None, "Expected None when the key is not found.");
}

