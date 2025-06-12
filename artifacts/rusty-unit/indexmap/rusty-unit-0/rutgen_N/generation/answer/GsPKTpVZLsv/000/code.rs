// Answer 0

#[test]
fn test_iter_mut() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        pub fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        pub fn iter_mut(&mut self) -> std::slice::IterMut<(K, V)> {
            self.entries.iter_mut()
        }
        
        pub fn push(&mut self, key: K, value: V) {
            self.entries.push((key, value));
        }
    }

    let mut map = TestMap::new();
    map.push(1, "a");
    map.push(2, "b");
    
    let mut iter = map.iter_mut();
    
    if let Some((key, value)) = iter.next() {
        *value = "c"; // Modify value
        assert_eq!(*value, "c");
        assert_eq!(*key, 1);
    }

    if let Some((key, value)) = iter.next() {
        assert_eq!(*value, "b");
        assert_eq!(*key, 2);
    }

    assert!(iter.next().is_none()); // Ensure no more elements
}

