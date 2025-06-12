// Answer 0

#[test]
fn test_remove_entry() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }
    
    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        fn insert(&mut self, key: K, value: V) {
            self.entries.push((key, value));
        }

        fn swap_remove_entry(&mut self, index: usize) -> (K, V) {
            self.entries.swap_remove(index)
        }

        fn remove_entry(&mut self, index: usize) -> (K, V) {
            self.swap_remove_entry(index)
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    let (key, value) = map.remove_entry(1);
    assert_eq!(key, 2);
    assert_eq!(value, "two");

    let (key, value) = map.remove_entry(1);
    assert_eq!(key, 3);
    assert_eq!(value, "three");

    let (key, value) = map.remove_entry(0);
    assert_eq!(key, 1);
    assert_eq!(value, "one");
}

#[test]
#[should_panic]
fn test_remove_entry_out_of_bounds() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }
    
    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        fn insert(&mut self, key: K, value: V) {
            self.entries.push((key, value));
        }

        fn swap_remove_entry(&mut self, index: usize) -> (K, V) {
            self.entries.swap_remove(index)
        }

        fn remove_entry(&mut self, index: usize) -> (K, V) {
            self.swap_remove_entry(index)
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "one");

    // This should panic because we're trying to remove an entry at an index that does not exist
    map.remove_entry(10);
}

