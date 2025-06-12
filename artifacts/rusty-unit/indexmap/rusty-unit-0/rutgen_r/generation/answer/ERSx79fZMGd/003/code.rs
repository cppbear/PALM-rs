// Answer 0

#[test]
#[should_panic]
fn test_insert_bulk_no_grow_panic_on_insufficient_capacity() {
    struct Bucket<K, V> {
        hash: HashWrapper<K>,
        value: V,
    }

    struct HashWrapper<K> {
        key: K,
    }

    struct Indices {
        entries: Vec<Bucket<i32, i32>>,
        capacity: usize,
    }

    impl Indices {
        fn new(capacity: usize) -> Self {
            Self {
                entries: Vec::with_capacity(capacity),
                capacity,
            }
        }

        fn capacity(&self) -> usize {
            self.capacity
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn insert_unique(&mut self, _hash: &HashWrapper<i32>, _index: usize, _f: fn() -> !) {
            // Simulate insertion
            if self.entries.len() < self.capacity {
                self.entries.push(Bucket {
                    hash: HashWrapper { key: 0 },
                    value: 0,
                });
            } else {
                _f(); // Force an unreachable call if insertion fails
            }
        }
    }

    let mut indices = Indices::new(3);  // Set capacity to 3
    indices.entries.push(Bucket { hash: HashWrapper { key: 1 }, value: 10 });
    indices.entries.push(Bucket { hash: HashWrapper { key: 2 }, value: 20 });
    
    let entries_to_insert = vec![
        Bucket { hash: HashWrapper { key: 3 }, value: 30 },
        Bucket { hash: HashWrapper { key: 4 }, value: 40 },
        Bucket { hash: HashWrapper { key: 5 }, value: 50 },
    ];

    insert_bulk_no_grow(&mut indices, &entries_to_insert); // This should panic
}

