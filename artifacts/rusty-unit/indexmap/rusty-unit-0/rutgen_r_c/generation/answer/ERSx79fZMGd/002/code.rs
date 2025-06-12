// Answer 0

#[test]
fn test_insert_bulk_no_grow() {
    struct MockIndices {
        capacity: usize,
        len: usize,
    }

    impl MockIndices {
        fn new(capacity: usize) -> Self {
            Self { capacity, len: 0 }
        }

        fn capacity(&self) -> usize {
            self.capacity
        }

        fn len(&self) -> usize {
            self.len
        }

        fn insert_unique(&mut self, _hash: u64, _index: usize, _f: fn(&mut usize)) {
            self.len += 1;  // Simulating successful insertion
        }
    }

    let mut indices = MockIndices::new(3);
    let entries = [
        Bucket { hash: HashValue(1), key: "key1", value: "value1" },
        Bucket { hash: HashValue(2), key: "key2", value: "value2" },
    ];

    // The capacity minus the length should equal the entries length (2)
    assert!(indices.capacity() - indices.len() == entries.len());
    
    insert_bulk_no_grow(&mut indices, &entries);
    
    // Ensure that the insertions have worked
    assert_eq!(indices.len(), 2);
}

#[should_panic]
#[test]
fn test_insert_bulk_no_grow_panic_not_enough_capacity() {
    struct MockIndices {
        capacity: usize,
        len: usize,
    }

    impl MockIndices {
        fn new(capacity: usize) -> Self {
            Self { capacity, len: 0 }
        }

        fn capacity(&self) -> usize {
            self.capacity
        }

        fn len(&self) -> usize {
            self.len
        }

        fn insert_unique(&mut self, _hash: u64, _index: usize, _f: fn(&mut usize)) {
            self.len += 1;  // Simulating successful insertion
        }
    }

    let mut indices = MockIndices::new(1);
    let entries = [
        Bucket { hash: HashValue(1), key: "key1", value: "value1" },
        Bucket { hash: HashValue(2), key: "key2", value: "value2" },
    ];

    // This condition fails as `2 > 1 - 0`
    insert_bulk_no_grow(&mut indices, &entries);
}

