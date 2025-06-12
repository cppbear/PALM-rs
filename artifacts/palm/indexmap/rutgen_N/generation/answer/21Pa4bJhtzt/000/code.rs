// Answer 0

#[test]
fn test_reserve_no_growth_needed() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
        indices: Vec<usize>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                entries: Vec::new(),
                indices: Vec::new(),
            }
        }

        fn reserve(&mut self, additional: usize) {
            self.indices.reserve(additional);
            if additional > self.entries.capacity() - self.entries.len() {
                self.reserve_entries(additional);
            }
        }

        fn reserve_entries(&mut self, additional: usize) {
            self.entries.reserve(additional);
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn capacity(&self) -> usize {
            self.entries.capacity()
        }
    }

    let mut map = TestMap::new();
    map.reserve(5);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 0);
}

#[test]
fn test_reserve_growth_needed() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
        indices: Vec<usize>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                entries: Vec::with_capacity(2),
                indices: Vec::new(),
            }
        }

        fn reserve(&mut self, additional: usize) {
            self.indices.reserve(additional);
            if additional > self.entries.capacity() - self.entries.len() {
                self.reserve_entries(additional);
            }
        }

        fn reserve_entries(&mut self, additional: usize) {
            self.entries.reserve(additional);
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn capacity(&self) -> usize {
            self.entries.capacity()
        }
    }

    let mut map = TestMap::new();
    map.entries.push((1, 1));
    map.entries.push((2, 2));
    map.reserve(3);
    assert_eq!(map.len(), 2);
    assert!(map.capacity() >= 5);
}

