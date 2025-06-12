// Answer 0

#[test]
fn test_from_hash_full_success() {
    struct TestMap<'a, K, V> {
        map: &'a std::collections::HashMap<K, V>,
    }

    impl<'a, K, V> TestMap<'a, K, V> {
        fn index_from_hash<F>(&self, _hash: u64, _is_match: F) -> Option<usize>
        where
            F: FnMut(&K) -> bool,
        {
            // Simplified example just for the purpose of the test
            for (index, (key, _)) in self.map.iter().enumerate() {
                if _is_match(key) {
                    return Some(index);
                }
            }
            None
        }

        pub fn from_hash_full<F>(
            self,
            hash: u64,
            is_match: F,
        ) -> Option<(usize, &'a K, &'a V)>
        where
            F: FnMut(&K) -> bool,
        {
            let i = self.index_from_hash(hash, is_match)?;
            let (key, value) = self.map.get_index(i).unwrap();
            Some((i, key, value))
        }
    }

    let mut map = std::collections::HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    
    let test_map = TestMap { map: &map };
    let result = test_map.from_hash_full(123, |&k| k == &"key1");
    
    assert!(result.is_some());
    if let Some((index, key, value)) = result {
        assert_eq!(index, 0);
        assert_eq!(key, &"key1");
        assert_eq!(value, &"value1");
    }
}

#[test]
fn test_from_hash_full_no_match() {
    struct TestMap<'a, K, V> {
        map: &'a std::collections::HashMap<K, V>,
    }

    impl<'a, K, V> TestMap<'a, K, V> {
        fn index_from_hash<F>(&self, _hash: u64, _is_match: F) -> Option<usize>
        where
            F: FnMut(&K) -> bool,
        {
            None
        }

        pub fn from_hash_full<F>(
            self,
            hash: u64,
            is_match: F,
        ) -> Option<(usize, &'a K, &'a V)>
        where
            F: FnMut(&K) -> bool,
        {
            let i = self.index_from_hash(hash, is_match)?;
            let (key, value) = self.map.get_index(i).unwrap();
            Some((i, key, value))
        }
    }

    let map = std::collections::HashMap::new();
    let test_map = TestMap { map: &map };
    let result = test_map.from_hash_full(123, |&k| k == &"nonexistent_key");
    
    assert!(result.is_none());
}

