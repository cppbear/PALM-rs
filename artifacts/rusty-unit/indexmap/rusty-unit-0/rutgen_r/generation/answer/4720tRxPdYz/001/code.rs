// Answer 0

#[test]
fn test_swap_remove_entry_non_empty_map() {
    struct TestMap<K, V> {
        data: indexmap::IndexMap<K, V>,
    }
    
    impl<K: std::hash::Hash + Eq + Clone, V> TestMap<K, V> {
        fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
            self.data.swap_remove_index(index).map(|(_, v)| (self.data.keys().nth(index).cloned().unwrap(), v))
        }
    }

    let mut map = TestMap { data: indexmap::IndexMap::new() };
    map.data.insert("key1", "value1");
    map.data.insert("key2", "value2");
    
    let entry = map.swap_remove_index(0);
    assert_eq!(entry, Some(("key1".to_string(), "value1".to_string())));
    assert_eq!(map.data.len(), 1);
}

#[test]
#[should_panic]
fn test_swap_remove_entry_out_of_bounds_index() {
    struct TestMap<K, V> {
        data: indexmap::IndexMap<K, V>,
    }
    
    impl<K: std::hash::Hash + Eq + Clone, V> TestMap<K, V> {
        fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
            self.data.swap_remove_index(index).map(|(_, v)| (self.data.keys().nth(index).cloned().unwrap(), v))
        }
    }

    let mut map = TestMap { data: indexmap::IndexMap::new() };
    map.data.insert("key1", "value1");

    let _entry = map.swap_remove_index(1); // This should panic
}

#[test]
fn test_swap_remove_entry_with_multiple_elements() {
    struct TestMap<K, V> {
        data: indexmap::IndexMap<K, V>,
    }
    
    impl<K: std::hash::Hash + Eq + Clone, V> TestMap<K, V> {
        fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
            self.data.swap_remove_index(index).map(|(_, v)| (self.data.keys().nth(index).cloned().unwrap(), v))
        }
    }

    let mut map = TestMap { data: indexmap::IndexMap::new() };
    map.data.insert("one", 1);
    map.data.insert("two", 2);
    map.data.insert("three", 3);

    let entry = map.swap_remove_index(1);
    assert_eq!(entry, Some(("two".to_string(), 2)));
    assert_eq!(map.data.len(), 2);
}

#[test]
fn test_swap_remove_entry_single_element() {
    struct TestMap<K, V> {
        data: indexmap::IndexMap<K, V>,
    }
    
    impl<K: std::hash::Hash + Eq + Clone, V> TestMap<K, V> {
        fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
            self.data.swap_remove_index(index).map(|(_, v)| (self.data.keys().nth(index).cloned().unwrap(), v))
        }
    }

    let mut map = TestMap { data: indexmap::IndexMap::new() };
    map.data.insert("only_key", "only_value");

    let entry = map.swap_remove_index(0);
    assert_eq!(entry, Some(("only_key".to_string(), "only_value".to_string())));
    assert_eq!(map.data.len(), 0);
}

#[test]
fn test_swap_remove_entry_empty_map() {
    struct TestMap<K, V> {
        data: indexmap::IndexMap<K, V>,
    }
    
    impl<K: std::hash::Hash + Eq + Clone, V> TestMap<K, V> {
        fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
            self.data.swap_remove_index(index).map(|(_, v)| (self.data.keys().nth(index).cloned().unwrap(), v))
        }
    }

    let mut map = TestMap { data: indexmap::IndexMap::new() };

    let entry = map.swap_remove_index(0); // This should panic
    assert!(entry.is_none());
}

