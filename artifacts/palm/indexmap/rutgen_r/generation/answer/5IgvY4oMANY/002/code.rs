// Answer 0

#[test]
fn test_swap_remove_finish_middle_element() {
    struct Entry<K, V> {
        key: K,
        value: V,
        hash: usize,
    }

    struct TestMap<K, V> {
        entries: Vec<Entry<K, V>>,
        indices: Vec<usize>,
    }

    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
                indices: Vec::new(),
            }
        }

        fn add(&mut self, key: K, value: V, hash: usize) {
            self.entries.push(Entry { key, value, hash });
            self.indices.push(self.entries.len() - 1);
        }

        fn swap_remove_finish(&mut self, index: usize) -> (K, V) {
            let entry = self.entries.swap_remove(index);
            if let Some(entry) = self.entries.get(index) {
                let last = self.entries.len();
                update_index(&mut self.indices, entry.hash, last, index);
            }
            (entry.key, entry.value)
        }
    }

    fn update_index(indices: &mut Vec<usize>, hash: usize, last: usize, index: usize) {
        // Simulating index update logic
        indices[hash % indices.len()] = index;
    }

    let mut map = TestMap::new();
    map.add(1, "a", 0);
    map.add(2, "b", 1);
    map.add(3, "c", 2);

    let (key, value) = map.swap_remove_finish(1);
    assert_eq!(key, 2);
    assert_eq!(value, "b");
}

#[test]
fn test_swap_remove_finish_last_element() {
    struct Entry<K, V> {
        key: K,
        value: V,
        hash: usize,
    }

    struct TestMap<K, V> {
        entries: Vec<Entry<K, V>>,
        indices: Vec<usize>,
    }

    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
                indices: Vec::new(),
            }
        }

        fn add(&mut self, key: K, value: V, hash: usize) {
            self.entries.push(Entry { key, value, hash });
            self.indices.push(self.entries.len() - 1);
        }

        fn swap_remove_finish(&mut self, index: usize) -> (K, V) {
            let entry = self.entries.swap_remove(index);
            if let Some(entry) = self.entries.get(index) {
                let last = self.entries.len();
                update_index(&mut self.indices, entry.hash, last, index);
            }
            (entry.key, entry.value)
        }
    }

    fn update_index(indices: &mut Vec<usize>, hash: usize, last: usize, index: usize) {
        indices[hash % indices.len()] = index;
    }

    let mut map = TestMap::new();
    map.add(1, "a", 0);
    map.add(2, "b", 1);

    let (key, value) = map.swap_remove_finish(1);
    assert_eq!(key, 2);
    assert_eq!(value, "b");
}

#[test]
#[should_panic]
fn test_swap_remove_finish_invalid_index() {
    struct Entry<K, V> {
        key: K,
        value: V,
        hash: usize,
    }

    struct TestMap<K, V> {
        entries: Vec<Entry<K, V>>,
        indices: Vec<usize>,
    }

    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
                indices: Vec::new(),
            }
        }

        fn add(&mut self, key: K, value: V, hash: usize) {
            self.entries.push(Entry { key, value, hash });
            self.indices.push(self.entries.len() - 1);
        }

        fn swap_remove_finish(&mut self, index: usize) -> (K, V) {
            let entry = self.entries.swap_remove(index);
            if let Some(entry) = self.entries.get(index) {
                let last = self.entries.len();
                update_index(&mut self.indices, entry.hash, last, index);
            }
            (entry.key, entry.value)
        }
    }

    fn update_index(indices: &mut Vec<usize>, hash: usize, last: usize, index: usize) {
        indices[hash % indices.len()] = index;
    }

    let mut map = TestMap::new();
    map.add(1, "a", 0);
    
    // This should panic because index 1 is out of bounds
    map.swap_remove_finish(1);
}

