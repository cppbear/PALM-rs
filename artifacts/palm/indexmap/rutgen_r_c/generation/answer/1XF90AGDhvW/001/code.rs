// Answer 0

#[test]
fn test_contains_key_existing_key() {
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

        fn contains_key<Q>(&self, key: &Q) -> bool
        where
            Q: ?Sized + Hash + Equivalent<i32>,
        {
            self.entries.iter().any(|(k, _)| k == key)
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "value1".to_string());
    map.insert(2, "value2".to_string());
    assert!(map.contains_key(&1));
}

#[test]
fn test_contains_key_non_existing_key() {
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

        fn contains_key<Q>(&self, key: &Q) -> bool
        where
            Q: ?Sized + Hash + Equivalent<i32>,
        {
            self.entries.iter().any(|(k, _)| k == key)
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "value1".to_string());
    map.insert(2, "value2".to_string());
    assert!(!map.contains_key(&3));
}

#[test]
fn test_contains_key_empty_map() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: vec![] }
        }

        fn contains_key<Q>(&self, key: &Q) -> bool
        where
            Q: ?Sized + Hash + Equivalent<i32>,
        {
            self.entries.iter().any(|(k, _)| k == key)
        }
    }

    let map = TestMap::new();
    assert!(!map.contains_key(&1));
}

