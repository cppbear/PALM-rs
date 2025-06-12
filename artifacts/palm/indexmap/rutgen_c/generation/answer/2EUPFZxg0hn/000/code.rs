// Answer 0

#[test]
fn test_capacity_empty() {
    struct TestHashTable {
        capacity: usize,
    }

    impl hash_table::HashTable<usize> {
        fn new() -> Self {
            TestHashTable { capacity: 0 }
        }
        
        fn capacity(&self) -> usize {
            self.capacity
        }
    }

    let map: IndexMapCore<usize, usize> = IndexMapCore::new();
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_capacity_with_entries() {
    struct TestHashTable {
        capacity: usize,
    }

    impl hash_table::HashTable<usize> {
        fn with_capacity(capacity: usize) -> Self {
            TestHashTable { capacity }
        }
        
        fn capacity(&self) -> usize {
            self.capacity
        }
    }

    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    assert_eq!(map.capacity(), 5);
}

#[test]
fn test_capacity_with_indices_and_entries() {
    struct TestHashTable {
        capacity: usize,
    }

    impl hash_table::HashTable<usize> {
        fn with_capacity(capacity: usize) -> Self {
            TestHashTable { capacity }
        }
        
        fn capacity(&self) -> usize {
            self.capacity
        }
    }

    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    assert_eq!(map.capacity(), 10);
    
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 1 });
    
    assert_eq!(map.capacity(), 10);
}

#[test]
fn test_capacity_indices_greater_than_entries() {
    struct TestHashTable {
        capacity: usize,
    }

    impl hash_table::HashTable<usize> {
        fn with_capacity(capacity: usize) -> Self {
            TestHashTable { capacity }
        }
        
        fn capacity(&self) -> usize {
            self.capacity
        }
    }

    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(8);
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 1 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 2 });

    assert_eq!(map.capacity(), 8);
}

#[test]
fn test_capacity_entries_greater_than_indices() {
    struct TestHashTable {
        capacity: usize,
    }

    impl hash_table::HashTable<usize> {
        fn with_capacity(capacity: usize) -> Self {
            TestHashTable { capacity }
        }
        
        fn capacity(&self) -> usize {
            self.capacity
        }
    }

    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(6);
    map.indices = TestHashTable::with_capacity(4); // simulate a smaller capacity for indices
    
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 1 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 2 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 3, value: 3 });
    
    assert_eq!(map.capacity(), 4);
}

