// Answer 0

#[test]
fn test_shift_remove_index_valid() {
    struct TestMap<K, V> {
        core: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            TestMap { core: Vec::new() }
        }

        fn insert(&mut self, key: K, value: V) {
            self.core.push((key, value));
        }

        fn len(&self) -> usize {
            self.core.len()
        }

        fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
            if index < self.len() {
                Some(self.core.remove(index))
            } else {
                None
            }
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    assert_eq!(map.shift_remove_index(1), Some((2, "two")));
    assert_eq!(map.len(), 2);
    assert_eq!(map.shift_remove_index(0), Some((1, "one")));
    assert_eq!(map.len(), 1);
    assert_eq!(map.shift_remove_index(0), Some((3, "three")));
    assert_eq!(map.len(), 0);
}

#[test]
#[should_panic]
fn test_shift_remove_index_out_of_bounds() {
    struct TestMap<K, V> {
        core: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            TestMap { core: Vec::new() }
        }

        fn insert(&mut self, key: K, value: V) {
            self.core.push((key, value));
        }

        fn len(&self) -> usize {
            self.core.len()
        }

        fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
            if index < self.len() {
                Some(self.core.remove(index))
            } else {
                panic!("Index out of bounds");
            }
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");
    
    let _ = map.shift_remove_index(3); // This should panic
}

#[test]
fn test_shift_remove_index_empty() {
    struct TestMap<K, V> {
        core: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            TestMap { core: Vec::new() }
        }

        fn len(&self) -> usize {
            self.core.len()
        }

        fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
            if index < self.len() {
                Some(self.core.remove(index))
            } else {
                None
            }
        }
    }

    let mut map = TestMap::new();
    assert_eq!(map.shift_remove_index(0), None); // Trying to remove from empty map
}

