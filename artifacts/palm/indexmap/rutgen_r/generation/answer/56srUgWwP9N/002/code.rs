// Answer 0

#[test]
fn test_from_hash_full_some_result() {
    struct TestMap {
        map: std::collections::HashMap<u64, (String, String)>,
        keys: Vec<u64>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut map = std::collections::HashMap::new();
            map.insert(1, ("key1".to_string(), "value1".to_string()));
            map.insert(2, ("key2".to_string(), "value2".to_string()));
            let keys = vec![1, 2];
            Self { map, keys }
        }

        fn index_from_hash<F>(&self, hash: u64, is_match: F) -> Option<usize>
        where
            F: FnMut(&u64) -> bool,
        {
            let mut is_match = is_match;
            self.keys.iter().position(|key| is_match(key))
        }

        fn get_index(&self, index: usize) -> Option<(&u64, &String)> {
            let key = self.keys.get(index)?;
            let value = self.map.get(key)?;
            Some((key, &value.0))
        }
    }

    let test_map = TestMap::new();
    let hash: u64 = 1;

    let result = test_map.from_hash_full(hash, |k| *k == hash);
    assert_eq!(result, Some((0, "key1", "value1")));
}

#[test]
#[should_panic(expected = "None")]
fn test_from_hash_full_none_result() {
    struct TestMap {
        map: std::collections::HashMap<u64, (String, String)>,
        keys: Vec<u64>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut map = std::collections::HashMap::new();
            map.insert(1, ("key1".to_string(), "value1".to_string()));
            let keys = vec![1];
            Self { map, keys }
        }

        fn index_from_hash<F>(&self, hash: u64, is_match: F) -> Option<usize>
        where
            F: FnMut(&u64) -> bool,
        {
            let mut is_match = is_match;
            self.keys.iter().position(|key| is_match(key))
        }

        fn get_index(&self, index: usize) -> Option<(&u64, &String)> {
            // Simulate a case where the index is valid but no value exists for it
            None
        }
    }

    let test_map = TestMap::new();
    let hash: u64 = 1;

    // This will panic because get_index returns None.
    let _result = test_map.from_hash_full(hash, |k| *k == hash);
}

