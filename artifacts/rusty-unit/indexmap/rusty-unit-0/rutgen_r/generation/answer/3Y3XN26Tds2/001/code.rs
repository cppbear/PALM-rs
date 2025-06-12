// Answer 0

#[test]
fn test_sort_by_cached_key() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        fn add_entry(&mut self, key: i32, value: String) {
            self.entries.push((key, value));
        }

        fn sort_by_cached_key<T, F>(&mut self, mut sort_key: F)
        where
            T: Ord,
            F: FnMut(&i32, &String) -> T,
        {
            self.entries.sort_by_cached_key(move |a| sort_key(&a.0, &a.1));
        }
    }

    let mut map = TestMap::new();
    map.add_entry(3, "three".to_string());
    map.add_entry(1, "one".to_string());
    map.add_entry(2, "two".to_string());

    map.sort_by_cached_key(|key, _value| *key);

    assert_eq!(map.entries, vec![(1, "one".to_string()), (2, "two".to_string()), (3, "three".to_string())]);
}

#[test]
fn test_sort_by_cached_key_with_repeated_keys() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        fn add_entry(&mut self, key: i32, value: String) {
            self.entries.push((key, value));
        }

        fn sort_by_cached_key<T, F>(&mut self, mut sort_key: F)
        where
            T: Ord,
            F: FnMut(&i32, &String) -> T,
        {
            self.entries.sort_by_cached_key(move |a| sort_key(&a.0, &a.1));
        }
    }

    let mut map = TestMap::new();
    map.add_entry(3, "three".to_string());
    map.add_entry(2, "two".to_string());
    map.add_entry(2, "duplicate".to_string());
    map.add_entry(1, "one".to_string());

    map.sort_by_cached_key(|key, _value| *key);

    assert_eq!(map.entries, vec![(1, "one".to_string()), (2, "two".to_string()), (2, "duplicate".to_string()), (3, "three".to_string())]);
}

#[test]
#[should_panic]
fn test_sort_by_cached_key_panics_with_invalid_key() {
    struct TestMap {
        entries: Vec<(Option<i32>, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        fn add_entry(&mut self, key: Option<i32>, value: String) {
            self.entries.push((key, value));
        }

        fn sort_by_cached_key<T, F>(&mut self, mut sort_key: F)
        where
            T: Ord,
            F: FnMut(&Option<i32>, &String) -> T,
        {
            self.entries.sort_by_cached_key(move |a| sort_key(&a.0, &a.1));
        }
    }

    let mut map = TestMap::new();
    map.add_entry(None, "none".to_string());
    map.add_entry(Some(3), "three".to_string());

    // This line should cause a panic due to None being compared.
    map.sort_by_cached_key(|key, _value| key.unwrap());
}

#[test]
fn test_sort_by_cached_key_stable_sort() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        fn add_entry(&mut self, key: i32, value: String) {
            self.entries.push((key, value));
        }

        fn sort_by_cached_key<T, F>(&mut self, mut sort_key: F)
        where
            T: Ord,
            F: FnMut(&i32, &String) -> T,
        {
            self.entries.sort_by_cached_key(move |a| sort_key(&a.0, &a.1));
        }
    }

    let mut map = TestMap::new();
    map.add_entry(2, "b".to_string());
    map.add_entry(1, "a".to_string());
    map.add_entry(2, "c".to_string());

    map.sort_by_cached_key(|key, _value| *key);

    assert_eq!(map.entries, vec![(1, "a".to_string()), (2, "b".to_string()), (2, "c".to_string())]);
}

