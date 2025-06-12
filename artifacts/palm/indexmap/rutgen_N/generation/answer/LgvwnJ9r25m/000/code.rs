// Answer 0

#[test]
fn test_shift_remove() {
    struct TestMap {
        entries: Vec<(usize, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        fn insert(&mut self, key: usize, value: String) {
            self.entries.push((key, value));
        }

        fn shift_remove(&mut self, index: usize) -> Option<String> {
            if index >= self.entries.len() {
                return None;
            }
            let (_, value) = self.entries.remove(index);
            Some(value)
        }
    }

    let mut map = TestMap::new();
    map.insert(0, "value0".to_string());
    map.insert(1, "value1".to_string());
    map.insert(2, "value2".to_string());

    assert_eq!(map.shift_remove(1), Some("value1".to_string()));
    assert_eq!(map.entries.len(), 2);
    assert_eq!(map.entries[0], (0, "value0".to_string()));
    assert_eq!(map.entries[1], (2, "value2".to_string()));
}

#[test]
fn test_shift_remove_boundary() {
    struct TestMap {
        entries: Vec<(usize, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        fn insert(&mut self, key: usize, value: String) {
            self.entries.push((key, value));
        }

        fn shift_remove(&mut self, index: usize) -> Option<String> {
            if index >= self.entries.len() {
                return None;
            }
            let (_, value) = self.entries.remove(index);
            Some(value)
        }
    }

    let mut map = TestMap::new();
    map.insert(0, "value0".to_string());

    assert_eq!(map.shift_remove(0), Some("value0".to_string()));
    assert_eq!(map.entries.len(), 0);
    assert_eq!(map.shift_remove(0), None); // Trying to remove from an empty map
}

