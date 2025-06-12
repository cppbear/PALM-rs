// Answer 0

#[test]
fn test_sort_by_cached_key_stable_sorting() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: vec![] }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.entries.push((key, value));
        }

        fn sort_by_cached_key<F>(&mut self, mut sort_key: F)
        where
            F: FnMut(&i32, &String) -> i32,
        {
            self.entries.sort_by_cached_key(|(key, value)| sort_key(key, value));
        }
        
        fn as_slice(&self) -> &[(i32, String)] {
            &self.entries
        }
    }

    let mut map = TestMap::new();
    map.insert(3, "three".to_string());
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());

    map.sort_by_cached_key(|key, _| *key);

    let sorted_entries = map.as_slice();
    assert_eq!(sorted_entries[0], (1, "one".to_string()));
    assert_eq!(sorted_entries[1], (2, "two".to_string()));
    assert_eq!(sorted_entries[2], (3, "three".to_string()));
}

#[test]
fn test_sort_by_cached_key_stable_sorting_with_same_keys() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: vec![] }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.entries.push((key, value));
        }

        fn sort_by_cached_key<F>(&mut self, mut sort_key: F)
        where
            F: FnMut(&i32, &String) -> i32,
        {
            self.entries.sort_by_cached_key(|(key, value)| sort_key(key, value));
        }

        fn as_slice(&self) -> &[(i32, String)] {
            &self.entries
        }
    }

    let mut map = TestMap::new();
    map.insert(2, "two".to_string());
    map.insert(1, "one".to_string());
    map.insert(2, "duplicate".to_string());

    map.sort_by_cached_key(|key, _| *key);

    let sorted_entries = map.as_slice();
    assert_eq!(sorted_entries[0], (1, "one".to_string()));
    assert_eq!(sorted_entries[1], (2, "two".to_string()));
    assert_eq!(sorted_entries[2], (2, "duplicate".to_string()));
}

#[test]
fn test_sort_by_cached_key_empty_map() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: vec![] }
        }

        fn sort_by_cached_key<F>(&mut self, _sort_key: F)
        where
            F: FnMut(&i32, &String) -> i32,
        {
            // No operations needed for empty map
        }
        
        fn as_slice(&self) -> &[(i32, String)] {
            &self.entries
        }
    }

    let mut map = TestMap::new();
    map.sort_by_cached_key(|_, _| 0);
    assert!(map.as_slice().is_empty());
}

