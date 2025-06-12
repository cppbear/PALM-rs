// Answer 0

#[test]
fn test_push_entry_capacity_full() {
    use std::collections::VecDeque;

    struct HashValue(u64);
    
    #[derive(Debug)]
    struct Bucket<K, V> {
        hash: HashValue,
        key: K,
        value: V,
    }

    struct Entries<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> Entries<K, V> {
        fn new(capacity: usize) -> Self {
            Self { entries: Vec::with_capacity(capacity) }
        }

        fn borrow_mut(&mut self) -> &mut Self {
            self
        }

        fn reserve_entries(&mut self, additional: usize) {
            self.entries.reserve(additional);
        }

        fn push_entry(&mut self, hash: HashValue, key: K, value: V) {
            if self.entries.len() == self.entries.capacity() {
                self.borrow_mut().reserve_entries(1);
            }
            self.entries.push(Bucket { hash, key, value });
        }
    }

    // Setup: Create an Entries instance with a capacity of 2
    let mut entries = Entries::new(2);
    entries.push_entry(HashValue(1), "key1", "value1");
    entries.push_entry(HashValue(2), "key2", "value2");

    // Check the length and capacity are both 2 before pushing another entry
    assert_eq!(entries.entries.len(), 2);
    assert_eq!(entries.entries.capacity(), 2);

    // Action: Push a third entry to trigger the capacity check and reserve
    entries.push_entry(HashValue(3), "key3", "value3");

    // Assertions: Ensure the third entry is now present
    assert_eq!(entries.entries.len(), 3);
    assert_eq!(entries.entries.capacity(), 3); // Capacity should be expanded
    assert_eq!(entries.entries[2].key, "key3");
    assert_eq!(entries.entries[2].value, "value3");
}

