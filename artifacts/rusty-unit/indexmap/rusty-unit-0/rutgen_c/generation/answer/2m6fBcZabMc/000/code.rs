// Answer 0

#[test]
fn test_shrink_to() {
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }
    
    impl TestEntries {
        fn new() -> Self {
            TestEntries {
                entries: Vec::new(),
            }
        }

        fn push(&mut self, hash: HashValue, key: usize, value: usize) {
            self.entries.push(Bucket { hash, key, value });
        }
        
        fn len(&self) -> usize {
            self.entries.len()
        }

        fn shrink_to(&mut self, min_capacity: usize) {
            self.entries.truncate(min_capacity);
        }
    }

    // Define the Indices as a placeholder
    struct TestIndices {
        capacity: usize,
    }

    impl TestIndices {
        fn new() -> Self {
            TestIndices { capacity: 0 }
        }

        fn shrink_to(&mut self, min_capacity: usize, _: impl Fn(&usize) -> u64) {
            self.capacity = min_capacity;
        }
    }

    struct TestIndexMapCore {
        indices: TestIndices,
        entries: TestEntries,
    }

    impl TestIndexMapCore {
        fn new() -> Self {
            TestIndexMapCore {
                indices: TestIndices::new(),
                entries: TestEntries::new(),
            }
        }

        fn shrink_to(&mut self, min_capacity: usize) {
            self.indices.shrink_to(min_capacity, |i| i as u64);
            self.entries.shrink_to(min_capacity);
        }
    }

    let mut map = TestIndexMapCore::new();
    map.entries.push(HashValue::from(1), 1, 10);
    map.entries.push(HashValue::from(2), 2, 20);
    
    assert_eq!(map.entries.len(), 2);
    
    // Shrink to a smaller size
    map.shrink_to(1);
    
    assert_eq!(map.entries.len(), 1);
    
    // Shrink to zero size
    map.shrink_to(0);
    
    assert_eq!(map.entries.len(), 0);
} 

#[test]
fn test_shrink_to_beyond_existing_capacity() {
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }
    
    impl TestEntries {
        fn new() -> Self {
            TestEntries {
                entries: Vec::new(),
            }
        }

        fn push(&mut self, hash: HashValue, key: usize, value: usize) {
            self.entries.push(Bucket { hash, key, value });
        }
        
        fn len(&self) -> usize {
            self.entries.len()
        }

        fn shrink_to(&mut self, min_capacity: usize) {
            self.entries.truncate(min_capacity);
        }
    }

    // Define the Indices as a placeholder
    struct TestIndices {
        capacity: usize,
    }

    impl TestIndices {
        fn new() -> Self {
            TestIndices { capacity: 0 }
        }

        fn shrink_to(&mut self, min_capacity: usize, _: impl Fn(&usize) -> u64) {
            self.capacity = min_capacity;
        }
    }

    struct TestIndexMapCore {
        indices: TestIndices,
        entries: TestEntries,
    }

    impl TestIndexMapCore {
        fn new() -> Self {
            TestIndexMapCore {
                indices: TestIndices::new(),
                entries: TestEntries::new(),
            }
        }

        fn shrink_to(&mut self, min_capacity: usize) {
            self.indices.shrink_to(min_capacity, |i| i as u64);
            self.entries.shrink_to(min_capacity);
        }
    }

    let mut map = TestIndexMapCore::new();
    map.entries.push(HashValue::from(1), 1, 10);
    map.entries.push(HashValue::from(2), 2, 20);

    assert_eq!(map.entries.len(), 2);
    
    // Try to shrink to more than its current size, ideally should not panic
    map.shrink_to(3);

    assert_eq!(map.entries.len(), 2);
}

