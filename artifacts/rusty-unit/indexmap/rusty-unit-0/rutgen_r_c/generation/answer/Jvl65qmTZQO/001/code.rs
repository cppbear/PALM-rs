// Answer 0

#[test]
fn test_swap_remove_entry_existing() {
    struct TestMap {
        data: indexmap::IndexMap<String, i32, std::collections::hash_map::RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut data = indexmap::IndexMap::new();
            data.insert("one".to_string(), 1);
            data.insert("two".to_string(), 2);
            data.insert("three".to_string(), 3);
            Self { data }
        }

        fn swap_remove_entry<Q>(&mut self, key: &Q) -> Option<(String, i32)>
        where
            Q: ?Sized + Hash + indexmap::Equivalent<String>,
        {
            match self.data.swap_remove_full(key) {
                Some((_, key, value)) => Some((key, value)),
                None => None,
            }
        }
    }

    let mut map = TestMap::new();
    let result = map.swap_remove_entry(&"two".to_string());
    assert_eq!(result, Some(("two".to_string(), 2)));
}

#[test]
fn test_swap_remove_entry_non_existing() {
    struct TestMap {
        data: indexmap::IndexMap<String, i32, std::collections::hash_map::RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut data = indexmap::IndexMap::new();
            data.insert("one".to_string(), 1);
            data.insert("two".to_string(), 2);
            data.insert("three".to_string(), 3);
            Self { data }
        }

        fn swap_remove_entry<Q>(&mut self, key: &Q) -> Option<(String, i32)>
        where
            Q: ?Sized + Hash + indexmap::Equivalent<String>,
        {
            match self.data.swap_remove_full(key) {
                Some((_, key, value)) => Some((key, value)),
                None => None,
            }
        }
    }

    let mut map = TestMap::new();
    let result = map.swap_remove_entry(&"four".to_string());
    assert_eq!(result, None);
}

#[test]
fn test_swap_remove_entry_boundary() {
    struct TestMap {
        data: indexmap::IndexMap<String, i32, std::collections::hash_map::RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut data = indexmap::IndexMap::new();
            data.insert("boundary".to_string(), 100);
            Self { data }
        }

        fn swap_remove_entry<Q>(&mut self, key: &Q) -> Option<(String, i32)>
        where
            Q: ?Sized + Hash + indexmap::Equivalent<String>,
        {
            match self.data.swap_remove_full(key) {
                Some((_, key, value)) => Some((key, value)),
                None => None,
            }
        }
    }

    let mut map = TestMap::new();
    let result = map.swap_remove_entry(&"boundary".to_string());
    assert_eq!(result, Some(("boundary".to_string(), 100)));
}

