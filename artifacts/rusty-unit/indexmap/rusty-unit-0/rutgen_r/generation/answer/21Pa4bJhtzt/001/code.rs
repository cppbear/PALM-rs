// Answer 0

#[test]
fn test_reserve_exceeds_capacity() {
    struct TestIndexMap {
        indices: Vec<usize>,
        entries: Vec<usize>,
    }

    impl TestIndexMap {
        fn new() -> Self {
            TestIndexMap {
                indices: Vec::new(),
                entries: Vec::new(),
            }
        }

        fn reserve(&mut self, additional: usize) {
            self.indices.reserve(additional);
            if additional > self.entries.capacity() - self.entries.len() {
                self.entries.reserve(additional);
            }
        }
    }

    let mut map = TestIndexMap::new();
    map.entries.reserve(5); // Initial capacity of 5

    // Triggering the condition where additional > entries.capacity() - entries.len()
    let additional = 10; // additional = 10, current capacity = 5, current len = 0
    map.reserve(additional);

    assert_eq!(map.entries.capacity(), 15); // After reserving, capacity should increase
}

#[test]
#[should_panic]
fn test_reserve_panic_condition() {
    struct TestIndexMap {
        indices: Vec<usize>,
        entries: Vec<usize>,
    }

    impl TestIndexMap {
        fn new() -> Self {
            TestIndexMap {
                indices: Vec::new(),
                entries: Vec::new(),
            }
        }

        fn reserve(&mut self, additional: usize) {
            self.indices.reserve(additional);
            if additional > self.entries.capacity() - self.entries.len() {
                self.entries.reserve(additional);
            }
        }
    }

    let mut map = TestIndexMap::new();
    
    // This will panic since the capacity is 0 and we will try to ask for more space
    let additional = 1; // additional = 1, current capacity = 0, current len = 0
    map.reserve(additional);
}

